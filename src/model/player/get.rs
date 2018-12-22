use super::Player;
use crate::{error::PointercrateError, operation::Get, Result};
use diesel::{result::Error, PgConnection, RunQueryDsl};

impl<'a> Get<&'a str> for Player {
    fn get(name: &'a str, connection: &PgConnection) -> Result<Self> {
        match Player::by_name(name).first(connection) {
            Ok(player) => Ok(player),
            Err(Error::NotFound) =>
                Player::insert(connection, &name).map_err(PointercrateError::database),
            Err(err) => Err(PointercrateError::database(err)),
        }
    }
}
