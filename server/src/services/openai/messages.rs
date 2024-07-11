use std::env;
use async_openai::types::{ChatCompletionRequestAssistantMessage, ChatCompletionRequestMessage, ChatCompletionRequestSystemMessage, ChatCompletionRequestUserMessage, ChatCompletionRequestUserMessageContent};

fn powershell_messages() -> Vec<ChatCompletionRequestMessage> {
    vec![
        ChatCompletionRequestMessage::System(system_message("Correctly answer the asked question. Return 'Sorry, Can't answer that.' if the question isn't related to technology.")),
        ChatCompletionRequestMessage::User(user_message("get into a docker container.")),
        ChatCompletionRequestMessage::Assistant(assistant_message("`docker exec -it <container>`")),
        ChatCompletionRequestMessage::User(user_message("Check what's listening on a port.")),
        ChatCompletionRequestMessage::Assistant(assistant_message("`netstat -ano | findstr :<port>`")),
        ChatCompletionRequestMessage::User(user_message("How to ssh into a server with a specific file.")),
        ChatCompletionRequestMessage::Assistant(assistant_message("`ssh -i <file_path> <user>@<port>`")),
        ChatCompletionRequestMessage::User(user_message("How to set relative line numbers in vim.")),
        ChatCompletionRequestMessage::Assistant(assistant_message("`:set relativenumber`")),
        ChatCompletionRequestMessage::User(user_message("How to create alias?")),
        ChatCompletionRequestMessage::Assistant(assistant_message("`Set-Alias <new_command> <old_command>`")),
        ChatCompletionRequestMessage::User(user_message("Tail docker logs.")),
        ChatCompletionRequestMessage::Assistant(assistant_message("`docker logs -f mongodb`")),
        ChatCompletionRequestMessage::User(user_message("Forward port in kubectl.")),
        ChatCompletionRequestMessage::Assistant(assistant_message("`kubectl port-forward <pod_name> 8080:3000`")),
        ChatCompletionRequestMessage::User(user_message("Check if a port is accessible.")),
        ChatCompletionRequestMessage::Assistant(assistant_message("`Test-NetConnection -ComputerName <host_name> -Port <port>`")),
        ChatCompletionRequestMessage::User(user_message("Kill a process running on port 3000.")),
        ChatCompletionRequestMessage::Assistant(assistant_message("`Get-Process -Id (Get-NetTCPConnection -LocalPort 3000).OwningProcess | Stop-Process`")),
        ChatCompletionRequestMessage::User(user_message("Backup database from a mongodb container.")),
        ChatCompletionRequestMessage::Assistant(assistant_message("`docker exec -it mongodb bash -c \"mongoexport --db mongodb --collection collections --outdir backup\"`")),
        ChatCompletionRequestMessage::User(user_message("SSH Tunnel Remote Host port into a local port.")),
        ChatCompletionRequestMessage::Assistant(assistant_message("`ssh -L <local_port>:<remote_host>:<remote_port> <user>@<remote_host>`")),
        ChatCompletionRequestMessage::User(user_message("Copy local file to S3.")),
        ChatCompletionRequestMessage::Assistant(assistant_message("`aws s3 cp <local_file> s3://<bucket_name>/<remote_file>`")),
        ChatCompletionRequestMessage::User(user_message("Copy S3 file to local.")),
        ChatCompletionRequestMessage::Assistant(assistant_message("`aws s3 cp s3://<bucket_name>/<remote_file> <local_file>`")),
        ChatCompletionRequestMessage::User(user_message("Recursively remove a folder.")),
        ChatCompletionRequestMessage::Assistant(assistant_message("`Remove-Item -Recurse <folder_name>`")),
        ChatCompletionRequestMessage::User(user_message("Copy a file from local to ssh server.")),
        ChatCompletionRequestMessage::Assistant(assistant_message("`scp /path/to/file user@server:/path/to/destination`")),
        ChatCompletionRequestMessage::User(user_message("Download a file from a URL.")),
        ChatCompletionRequestMessage::Assistant(assistant_message("`Invoke-WebRequest -Uri <url> -OutFile <file_name>`")),
        ChatCompletionRequestMessage::User(user_message("Git commit with message.")),
        ChatCompletionRequestMessage::Assistant(assistant_message("`git commit -m \"my commit message\"`")),
        ChatCompletionRequestMessage::User(user_message("Give a user sudo permissions.")),
        ChatCompletionRequestMessage::Assistant(assistant_message("`Add-LocalGroupMember -Group \"Administrators\" -Member <user>`")),
        ChatCompletionRequestMessage::User(user_message("Check what's running on a port?")),
        ChatCompletionRequestMessage::Assistant(assistant_message("`Get-Process -Id (Get-NetTCPConnection -LocalPort <port>).OwningProcess`")),
        ChatCompletionRequestMessage::User(user_message("View last 5 files from history")),
        ChatCompletionRequestMessage::Assistant(assistant_message("`Get-History -Count 5`")),
        ChatCompletionRequestMessage::User(user_message("When was China founded?")),
        ChatCompletionRequestMessage::Assistant(assistant_message("Sorry, Can't answer that.")),
        ChatCompletionRequestMessage::User(user_message("Filter docker container with labels")),
        ChatCompletionRequestMessage::Assistant(assistant_message("`docker ps --filter \"label=<KEY>\"`")),
        ChatCompletionRequestMessage::User(user_message("When was Abraham Lincoln born?")),
        ChatCompletionRequestMessage::Assistant(assistant_message("Sorry, Can't answer that.")),
        ChatCompletionRequestMessage::User(user_message("Get into a running kubernetes pod")),
        ChatCompletionRequestMessage::Assistant(assistant_message("`kubectl exec -it <pod_name> bash`")),
        ChatCompletionRequestMessage::User(user_message("Capital city of Ukraine?")),
        ChatCompletionRequestMessage::Assistant(assistant_message("Sorry, Can't answer that.")),
    ]
}

fn unix_messages() -> Vec<ChatCompletionRequestMessage> {
    vec![
        ChatCompletionRequestMessage::System(system_message("Correctly answer the asked question. Return 'Sorry, Can't answer that.' if the question isn't related to technology.")),
        ChatCompletionRequestMessage::User(user_message("get into a docker container.")),
        ChatCompletionRequestMessage::Assistant(assistant_message("`docker exec -it mongodb`")),
        ChatCompletionRequestMessage::User(user_message("Check what's listening on a port.")),
        ChatCompletionRequestMessage::Assistant(assistant_message("`lsof -i tcp:4000`")),
        ChatCompletionRequestMessage::User(user_message("How to ssh into a server with a specific file.")),
        ChatCompletionRequestMessage::Assistant(assistant_message("`ssh -i ~/.ssh/id_rsa user@127.0.0.1`")),
        ChatCompletionRequestMessage::User(user_message("How to set relative line numbers in vim.")),
        ChatCompletionRequestMessage::Assistant(assistant_message("`:set relativenumber`")),
        ChatCompletionRequestMessage::User(user_message("How to create alias?")),
        ChatCompletionRequestMessage::Assistant(assistant_message("`alias my_command=\"my_real_command\"`")),
        ChatCompletionRequestMessage::User(user_message("Tail docker logs.")),
        ChatCompletionRequestMessage::Assistant(assistant_message("`docker logs -f mongodb`")),
        ChatCompletionRequestMessage::User(user_message("Forward port in kubectl.")),
        ChatCompletionRequestMessage::Assistant(assistant_message("`kubectl port-forward <pod_name> 8080:3000`")),
        ChatCompletionRequestMessage::User(user_message("Check if a port is accessible.")),
        ChatCompletionRequestMessage::Assistant(assistant_message("`nc -vz host port`")),
        ChatCompletionRequestMessage::User(user_message("Reverse SSH Tunnel Syntax.")),
        ChatCompletionRequestMessage::Assistant(assistant_message("`ssh -R <remote_port>:<local_host>:<local_port> <user>@<remote_host>`")),
        ChatCompletionRequestMessage::User(user_message("Kill a process running on port 3000.")),
        ChatCompletionRequestMessage::Assistant(assistant_message("`lsof -ti tcp:3000 | xargs kill`")),
        ChatCompletionRequestMessage::User(user_message("Backup database from a mongodb container.")),
        ChatCompletionRequestMessage::Assistant(assistant_message("`docker exec -it mongodb bash -c \"mongoexport --db mongodb --collection collections --outdir backup\"`")),
        ChatCompletionRequestMessage::User(user_message("SSH Tunnel Remote Host port into a local port.")),
        ChatCompletionRequestMessage::Assistant(assistant_message("`ssh -L <local_port>:<remote_host>:<remote_port> <user>@<remote_host>`")),
        ChatCompletionRequestMessage::User(user_message("Copy local file to S3.")),
        ChatCompletionRequestMessage::Assistant(assistant_message("`aws s3 cp <local_file> s3://<bucket_name>/<remote_file>`")),
        ChatCompletionRequestMessage::User(user_message("Copy S3 file to local.")),
        ChatCompletionRequestMessage::Assistant(assistant_message("`aws s3 cp s3://<bucket_name>/<remote_file> <local_file>`")),
        ChatCompletionRequestMessage::User(user_message("Recursively remove a folder.")),
        ChatCompletionRequestMessage::Assistant(assistant_message("`rm -rf <folder_name>`")),
        ChatCompletionRequestMessage::User(user_message("Copy a file from local to ssh server.")),
        ChatCompletionRequestMessage::Assistant(assistant_message("`scp /path/to/file user@server:/path/to/destination`")),
        ChatCompletionRequestMessage::User(user_message("Curl syntax with port.")),
        ChatCompletionRequestMessage::Assistant(assistant_message("`curl http://localhost:3000`")),
        ChatCompletionRequestMessage::User(user_message("Download a file from a URL with curl.")),
        ChatCompletionRequestMessage::Assistant(assistant_message("`curl -o <file_name> <URL>`")),
        ChatCompletionRequestMessage::User(user_message("Git commit with message.")),
        ChatCompletionRequestMessage::Assistant(assistant_message("`git commit -m \"my commit message\"`")),
        ChatCompletionRequestMessage::User(user_message("Give a user sudo permissions.")),
        ChatCompletionRequestMessage::Assistant(assistant_message("`sudo usermod -aG sudo <user>`")),
        ChatCompletionRequestMessage::User(user_message("Check what's running on a port?")),
        ChatCompletionRequestMessage::Assistant(assistant_message("`lsof -i tcp:<port>`")),
        ChatCompletionRequestMessage::User(user_message("View last 5 files from history")),
        ChatCompletionRequestMessage::Assistant(assistant_message("`history | tail -5`")),
        ChatCompletionRequestMessage::User(user_message("When was China founded?")),
        ChatCompletionRequestMessage::Assistant(assistant_message("Sorry, Can't answer that.")),
        ChatCompletionRequestMessage::User(user_message("Pass auth header with curl")),
        ChatCompletionRequestMessage::Assistant(assistant_message("`curl -H \"Authorization: Bearer <token>\" <URL>`")),
        ChatCompletionRequestMessage::User(user_message("Filter docker container with labels")),
        ChatCompletionRequestMessage::Assistant(assistant_message("`docker ps --filter \"label=<KEY>\"`")),
        ChatCompletionRequestMessage::User(user_message("When was Abraham Lincoln born?")),
        ChatCompletionRequestMessage::Assistant(assistant_message("Sorry, Can't answer that.")),
        ChatCompletionRequestMessage::User(user_message("Get into a running kubernetes pod")),
        ChatCompletionRequestMessage::Assistant(assistant_message("`kubectl exec -it <pod_name> bash`")),
        ChatCompletionRequestMessage::User(user_message("Capital city of Ukraine?")),
        ChatCompletionRequestMessage::Assistant(assistant_message("Sorry, Can't answer that.")),
    ]
}

fn system_message(content: &str) -> ChatCompletionRequestSystemMessage {
    ChatCompletionRequestSystemMessage {
        content: content.to_string(),
        name: None
    }
}

pub fn user_message(content: &str) -> ChatCompletionRequestUserMessage {
    ChatCompletionRequestUserMessage {
        content: ChatCompletionRequestUserMessageContent::Text(content.to_string()),
        name: None
    }
}

#[allow(deprecated)]
fn assistant_message(content: &str) -> ChatCompletionRequestAssistantMessage {
    ChatCompletionRequestAssistantMessage {
        content: Some(content.to_string()),
        name: None,
        tool_calls: None,
        function_call: None,
    }
}

pub fn default_messages() -> Vec<ChatCompletionRequestMessage> {
    let platform = env::consts::OS;

    match platform {
        "windows" => powershell_messages(),
        _ => unix_messages(),
    }
}
