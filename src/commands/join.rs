use serenity::builder::CreateCommand;
use serenity::model::application::CommandInteraction;
use serenity::client::Context;

pub async fn run(cmd: &CommandInteraction, ctx: &Context) -> String {
    let guild_id = match cmd.guild_id {
        Some(id) => id,
        None => {
            println!("no gid found");
            return "failed to find guild id".to_string();
        }
    };
    
    let channel_id = match ctx.cache.guild(guild_id) {
        Some(guild) => {
            guild
                .voice_states
                .get(&cmd.user.id)
                .and_then(|voice_state| voice_state.channel_id)
        }
        None => {
            println!("guild not in cache");
            return "failed to find guild".to_string();
        }
    };

    let connect_to = match channel_id {
        Some(channel) => channel,
        None => {
            println!("user not in channel");
            return "you aren't in a voice channel".to_string();
        }
    };

    let vmanager = songbird::get(ctx)
        .await
        .expect("Songbird voice client placed in at init.")
        .clone();

    vmanager.join(guild_id, connect_to).await;

    connect_to.to_string()
}

pub fn register() -> CreateCommand {
    CreateCommand::new("join").description("Join voice call")
}
