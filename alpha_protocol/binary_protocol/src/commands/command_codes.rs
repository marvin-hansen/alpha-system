// 0 - 99:  system codes
pub const HEARTBEAT_LABEL: &str = "system.heartbeat";
pub const HEARTBEAT_CODE: u16 = 1;
pub const PING_LABEL: &str = "system.ping";
pub const PING_CODE: u16 = 2;
pub const PONG_LABEL: &str = "system.pong";
pub const PONG_CODE: u16 = 3;
pub const REGISTER_CLIENT_LABEL: &str = "system.register_client";
pub const REGISTER_CLIENT_CODE: u16 = 4;
pub const UNREGISTER_CLIENT_LABEL: &str = "system.unregister_client";
pub const UNREGISTER_CLIENT_CODE: u16 = 5;

pub const SERVER_SHUTDOWN_LABEL: &str = "system.server_shutdown";
pub const SERVER_SHUTDOWN_CODE: u16 = 10;

// 100 - 199: message codes
pub const POLL_MESSAGES_LABEL: &str = "message.poll";
pub const POLL_MESSAGES_CODE: u16 = 100;
pub const SEND_MESSAGES_LABEL: &str = "message.send";
pub const SEND_MESSAGES_CODE: u16 = 101;
pub const FORWARD_MESSAGES_LABEL: &str = "message.forward";
pub const FORWARD_MESSAGES_CODE: u16 = 102;
pub const FLUSH_UNSAVED_BUFFER_LABEL: &str = "message.flush_unsaved_buffer";
pub const FLUSH_UNSAVED_BUFFER_CODE: u16 = 103;

// 200 - 299: stream codes
pub const GET_STREAM_LABEL: &str = "stream.get";
pub const GET_STREAM_CODE: u16 = 200;
pub const GET_ALL_STREAMS_LABEL: &str = "stream.list";
pub const GET_ALL_STREAMS_CODE: u16 = 201;
pub const CREATE_STREAM_LABEL: &str = "stream.create";
pub const CREATE_STREAM_CODE: u16 = 202;
pub const DELETE_STREAM_LABEL: &str = "stream.delete";
pub const DELETE_STREAM_CODE: u16 = 203;
pub const UPDATE_STREAM_LABEL: &str = "stream.update";
pub const UPDATE_STREAM_CODE: u16 = 204;
pub const PURGE_STREAM_LABEL: &str = "stream.purge";
pub const PURGE_STREAM_CODE: u16 = 205;

// 300 - 399: topic codes
pub const GET_TOPIC_LABEL: &str = "topic.get";
pub const GET_TOPIC_CODE: u16 = 300;
pub const GET_ALL_TOPICS_LABEL: &str = "topic.list";
pub const GET_ALL_TOPICS_CODE: u16 = 301;
pub const CREATE_TOPIC_LABEL: &str = "topic.create";
pub const CREATE_TOPIC_CODE: u16 = 302;
pub const DELETE_TOPIC_LABEL: &str = "topic.delete";
pub const DELETE_TOPIC_CODE: u16 = 303;
pub const UPDATE_TOPIC_LABEL: &str = "topic.update";
pub const UPDATE_TOPIC_CODE: u16 = 304;
pub const PURGE_TOPIC_LABEL: &str = "topic.purge";
pub const PURGE_TOPIC_CODE: u16 = 305;

// Add a static function that takes the command code and returns the command label
pub fn get_command_label(command_code: u16) -> &'static str {
    match command_code {
        HEARTBEAT_CODE => HEARTBEAT_LABEL,
        PING_CODE => PING_LABEL,
        PONG_CODE => PONG_LABEL,
        REGISTER_CLIENT_CODE => REGISTER_CLIENT_LABEL,
        UNREGISTER_CLIENT_CODE => UNREGISTER_CLIENT_LABEL,
        POLL_MESSAGES_CODE => POLL_MESSAGES_LABEL,
        SEND_MESSAGES_CODE => SEND_MESSAGES_LABEL,
        FORWARD_MESSAGES_CODE => FORWARD_MESSAGES_LABEL,
        FLUSH_UNSAVED_BUFFER_CODE => FLUSH_UNSAVED_BUFFER_LABEL,
        GET_STREAM_CODE => GET_STREAM_LABEL,
        GET_ALL_STREAMS_CODE => GET_ALL_STREAMS_LABEL,
        CREATE_STREAM_CODE => CREATE_STREAM_LABEL,
        DELETE_STREAM_CODE => DELETE_STREAM_LABEL,
        UPDATE_STREAM_CODE => UPDATE_STREAM_LABEL,
        PURGE_STREAM_CODE => PURGE_STREAM_LABEL,
        GET_TOPIC_CODE => GET_TOPIC_LABEL,
        GET_ALL_TOPICS_CODE => GET_ALL_TOPICS_LABEL,
        CREATE_TOPIC_CODE => CREATE_TOPIC_LABEL,
        DELETE_TOPIC_CODE => DELETE_TOPIC_LABEL,
        UPDATE_TOPIC_CODE => UPDATE_TOPIC_LABEL,
        PURGE_TOPIC_CODE => PURGE_TOPIC_LABEL,
        _ => "Unknown command code",
    }
}
