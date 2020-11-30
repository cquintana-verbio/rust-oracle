#!/bin/bash

set -ev

"$ORACLE_HOME/bin/sqlplus" / as sysdba @oracle/tests/SetupTest.sql < /dev/null

