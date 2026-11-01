use chrono::Local;
use std::error::Error;
use crate::connection_provider::ConnectionProviderPostgres;

// ==========================================
// 2. LogSinkDb Class: prior usage, let PostgreSql administrator(i.e. postgres) run this script:
// ==========================================
/*
-- Connect to 'postgres' or 'template1' first, then:
--DROP DATABASE IF EXISTS logging WITH (FORCE);

-- =========================================================================
-- DBA SETUP SCRIPT FOR LOGGING INFRASTRUCTURE
-- Run this script as a superuser (e.g., postgres) prior to application startup.
-- =========================================================================

-- 1. Create the dedicated schema (and optional database if creating from scratch)
-- CREATE DATABASE logging; -- Uncomment if a brand-new database is desired
CREATE SCHEMA IF NOT EXISTS logging;

-- 2. Create the dedicated least-privilege user
DO $$
BEGIN
   IF NOT EXISTS (SELECT FROM pg_catalog.pg_roles WHERE rolname = 'app_user') THEN
      CREATE ROLE app_user LOGIN PASSWORD 'application-user';
   END IF;
END
$$;

-- 3. Revoke default public schema access and grant strict isolation to logging schema
REVOKE ALL ON SCHEMA public FROM PUBLIC;
GRANT USAGE, CREATE ON SCHEMA logging TO app_user;

-- Ensure app_user cannot traverse or access other schemas (like public)
REVOKE ALL ON SCHEMA public FROM app_user;


-- =========================================================================
-- 4. FUNCTION: logging.create_log_table(text)
-- =========================================================================
CREATE OR REPLACE FUNCTION logging.create_log_table(logname text)
    RETURNS void
    LANGUAGE 'plpgsql'
    SECURITY DEFINER -- Ensures DDL executes with owner privileges safely
    SET search_path = logging, pg_temp
AS $BODY$
DECLARE
    table_exists text;
    cmd text;
BEGIN
    -- Strong check using format('%I', ...) to prevent SQL injection
    SELECT TO_REGCLASS(format('%I', logname)) INTO table_exists;

    IF table_exists IS NULL THEN
        -- Construct the dynamic SQL statement to create the table in 'logging' schema
        cmd := format(
            'CREATE TABLE logging.%I ('
            '    id SERIAL PRIMARY KEY, '
            '    when_time TIMESTAMPTZ NOT NULL, '
            '    row_nature CHAR(3) NOT NULL, '
            '    stack_depth VARCHAR(5) NOT NULL, '
            '    function_name VARCHAR(50) NOT NULL, '
            '    log_content VARCHAR(7919) NOT NULL'
            ')',
            logname
        );

        -- Execute the dynamic command
        EXECUTE cmd;
    END IF;
END;
$BODY$;

ALTER FUNCTION logging.create_log_table(text) OWNER TO postgres;


-- =========================================================================
-- 5. FUNCTION: logging.trace(text, text, text, text, text, text)
-- =========================================================================
CREATE OR REPLACE FUNCTION logging.trace(
    logname text,
    when_time text,
    row_nature text,
    stack_depth text,
    function_name text,
    log_content text)
    RETURNS void
    LANGUAGE 'plpgsql'
    SECURITY DEFINER
    SET search_path = logging, pg_temp
AS $BODY$
DECLARE
    table_exists text;
    cmd text;
BEGIN
    -- Strong check using format('%I', ...) to verify table existence safely
    SELECT TO_REGCLASS(format('%I', logname)) INTO table_exists;

    IF table_exists IS NOT NULL THEN
        -- Construct the dynamic INSERT statement safely using %I for table name 
        -- and %L for string literals / timestamp formatting
        cmd := format(
            'INSERT INTO logging.%I (when_time, row_nature, stack_depth, function_name, log_content) '
            'VALUES (%L::TIMESTAMPTZ, %L, %L, %L, %L)',
            logname,
            when_time,
            row_nature,
            stack_depth,
            function_name,
            log_content
        );

        -- Execute the dynamic insert command
        EXECUTE cmd;
    ELSE
        -- Required table not found -> do nothing
        NULL;
    END IF;
END;
$BODY$;

ALTER FUNCTION logging.trace(text, text, text, text, text, text) OWNER TO postgres;
GRANT EXECUTE ON FUNCTION logging.trace(text, text, text, text, text, text) TO app_user;


-- =========================================================================
-- 6. PERMISSIONS ASSIGNMENT
-- =========================================================================
REVOKE EXECUTE ON FUNCTION logging.create_log_table(text) FROM PUBLIC;
GRANT EXECUTE ON FUNCTION logging.create_log_table(text) TO app_user;
GRANT EXECUTE ON FUNCTION logging.create_log_table(text) TO postgres;

REVOKE EXECUTE ON FUNCTION logging.trace(text, text, text, text, text, text) FROM PUBLIC;
GRANT EXECUTE ON FUNCTION logging.trace(text, text, text, text, text, text) TO app_user;
GRANT EXECUTE ON FUNCTION logging.trace(text, text, text, text, text, text) TO postgres;

*/
pub struct LogSinkDb {
    provider: ConnectionProviderPostgres,
    log_table_name: String,
}

impl LogSinkDb {
    // Constructor (associated function)
    pub async fn new(table_name: &str, server_name: &str, config_path: &str) -> Result<Self, Box<dyn Error>> {
        let mut provider = ConnectionProviderPostgres::new(server_name, config_path).await?;

        if let Some(_conn) = provider.get_connection() {
            println!("Database connection initialized for table: {}", table_name);
        }

        let log_table_name = table_name.to_string();

        // Call create_log_table(?) once upon initialization
        let create_table_sql = "SELECT logging.create_log_table($1)";
        if let Some(conn) = provider.get_connection() {
            sqlx::query(create_table_sql)
                .bind(&log_table_name)
                .execute(conn)
                .await?;
            println!("-> Checked/Created log table: {}", log_table_name);
        } else {
            return Err("Failed to obtain active connection for table creation.".into());
        }        

        Ok(Self {
            provider,
            log_table_name: table_name.to_string(),
        })
    }

    // Trace method optimized for numerical analysis loops
    pub async fn trace(
        &mut self,
        row_nature: &str,    // 3 chars {o,t,c} or {INF,DBG,..}
        stack_depth: &str,    // call stack depth
        function_name: &str,  // calling method name
        log_content: &str,    // message content
    ) -> Result<(), Box<dyn Error>> {
        let trace_sql = "SELECT logging.trace($1, $2, $3, $4, $5, $6)";
        let current_formatted_date = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();

        if let Some(conn) = self.provider.get_connection() {
            sqlx::query(trace_sql)
                .bind(&self.log_table_name)
                .bind(&current_formatted_date)
                .bind(row_nature)
                .bind(stack_depth)
                .bind(function_name)
                .bind(log_content)
                .execute(conn)
                .await?;
        } else {
            println!("Failed to write trace log: Connection is closed or unavailable.");
        }

        Ok(())
    }

    // Explicit dispose method
    pub async fn dispose(&mut self) {
        self.provider.close_connection().await;
    }
}