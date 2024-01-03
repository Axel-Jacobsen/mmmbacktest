use rusqlite::{params, Connection, Result};

use crate::data_types as dt;

// https://chat.openai.com/share/d2e2e7a3-80ed-4502-b090-9eb6237c5c74
// Geez this is harder than I thought. I hope that throwing it all into a sqlite db
// won't be too painful. I wish I knew more about this. Enough for today. Why is sql
// serialization so ugly?? I hate that I have to go convert json to a rust struct to
// convert it to a sqlite struct, just to be converted back to a rust struct and then
// serialized back into json. That's awful.
//
// Actually, there *has* to be a better way. This is insane. Maybe just making my own index is
// fine.
