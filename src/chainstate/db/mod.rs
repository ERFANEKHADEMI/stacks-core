/*
 copyright: (c) 2013-2018 by Blockstack PBC, a public benefit corporation.

 This file is part of Blockstack.

 Blockstack is free software. You may redistribute or modify
 it under the terms of the GNU General Public License as published by
 the Free Software Foundation, either version 3 of the License or
 (at your option) any later version.

 Blockstack is distributed in the hope that it will be useful,
 but WITHOUT ANY WARRANTY, including without the implied warranty of
 MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
 GNU General Public License for more details.

 You should have received a copy of the GNU General Public License
 along with Blockstack. If not, see <http://www.gnu.org/licenses/>.
*/

pub mod burndb;

use std::fmt;
use std::error;

use rusqlite;
use rusqlite::Error as sqlite_error;

use serde_json::Error as serde_error;

use burnchains::{Txid, Hash160};

#[derive(Debug)]
pub enum Error {
    /// Not implemented 
    NotImplemented,
    /// Database doesn't exist
    NoDBError,
    /// DB connection error 
    ConnectionError,
    /// Read-only and tried to write
    ReadOnly,
    /// Transaction already in progress
    TransactionInProgress,
    /// No transaction in progress
    NoTransaction,
    /// Type error -- can't represent the given data in the database 
    TypeError,
    /// Serialization error -- can't serialize data
    SerializationError(serde_error),
    /// Sqlite3 error
    SqliteError(sqlite_error)
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::NotImplemented => f.write_str(error::Error::description(self)),
            Error::NoDBError => f.write_str(error::Error::description(self)),
            Error::ConnectionError => f.write_str(error::Error::description(self)),
            Error::ReadOnly => f.write_str(error::Error::description(self)),
            Error::TransactionInProgress => f.write_str(error::Error::description(self)),
            Error::NoTransaction => f.write_str(error::Error::description(self)),
            Error::TypeError => f.write_str(error::Error::description(self)),
            Error::SerializationError(ref e) => fmt::Display::fmt(e, f),
            Error::SqliteError(ref e) => fmt::Display::fmt(e, f)
        }
    }
}

impl error::Error for Error {
    fn cause(&self) -> Option<&error::Error> {
        match *self {
            Error::NotImplemented => None,
            Error::NoDBError => None,
            Error::ConnectionError => None,
            Error::ReadOnly => None,
            Error::TransactionInProgress => None,
            Error::NoTransaction => None,
            Error::TypeError => None,
            Error::SerializationError(ref e) => Some(e),
            Error::SqliteError(ref e) => Some(e)
        }
    }

    fn description(&self) -> &str {
        match *self {
            Error::NotImplemented => "Not implemented",
            Error::NoDBError => "Database does not exist",
            Error::ConnectionError => "Failed to connect to database",
            Error::ReadOnly => "Database is opened read-only",
            Error::TransactionInProgress => "Transaction already in progress",
            Error::NoTransaction => "No transaction active",
            Error::TypeError => "Invalid or unrepresentable database type",
            Error::SerializationError(ref e) => e.description(),
            Error::SqliteError(ref e) => e.description()
        }
    }
}

pub trait ChainstateDB {
    fn backup(backup_path: &String) -> Result<(), Error>;
}

