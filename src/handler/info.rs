// Copyright (C) 2026 The pgmoneta community
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

use super::PgmonetaClient;
use super::PgmonetaHandler;
use crate::constant::Sort;
use rmcp::ErrorData as McpError;
use rmcp::schemars;

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct InfoRequest {
    pub username: String,
    pub server: String,
    pub backup_id: String,
}

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct ListBackupsRequest {
    pub username: String,
    pub server: String,
    pub sort: Option<String>,
}

impl PgmonetaHandler {
    pub(super) async fn _get_backup_info(&self, request: InfoRequest) -> Result<String, McpError> {
        PgmonetaClient::request_backup_info(&request.username, &request.server, &request.backup_id)
            .await
            .map_err(|e| {
                McpError::internal_error(
                    format!("Failed to retrieve backup information: {:?}", e),
                    None,
                )
            })
    }

    pub(super) async fn _list_backups(
        &self,
        request: ListBackupsRequest,
    ) -> Result<String, McpError> {
        let sort = request.sort.unwrap_or(Sort::ASC.to_string());
        PgmonetaClient::request_list_backups(&request.username, &request.server, &sort)
            .await
            .map_err(|e| McpError::internal_error(format!("Failed to list backups: {:?}", e), None))
    }
}
