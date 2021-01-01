use std::collections::HashMap;
use std::env;

use serenity::{
    async_trait,
    model::{
        channel::Channel, channel::Reaction, channel::ReactionType,
        gateway::Ready,
    },
    prelude::*,
};

const CHANNEL_TO_WATCH: &'static str = "channel-management";

enum Action {
    AddRole,
    RemoveRole,
}

struct Handler {
    cache: Mutex<Option<HashMap<String, u64>>>,
}

impl Handler {
    async fn handle_reaction(&self, ctx: Context, reaction: Reaction, action: Action) {
        // We have to have a channel so we can track where this reaction occured
        // We can clone http because it is an `Arc`, so it's basically free
        if let Ok(channel) = reaction.channel(ctx.http.clone()).await {
            // We only want to match on `Guild` because we don't care about private chats or anything
            match channel {
                Channel::Guild(gc) => {
                    // We are only going to be watching this specific channel for reactions, ignore everything else
                    if gc.name() == CHANNEL_TO_WATCH {
                        // We want to make sure we can get the message from the channel because we use that to parse
                        // out which roles we assign dynamically, if we can't get it, just return because there is
                        // nothing the bot can do
                        if let Ok(msg) = reaction.message(ctx.http.clone()).await {
                            // Pull out `emoji`, `user_id`, and `guild_id` from a partial destructure of the reaction
                            // These are all the things we need for later and consumes the reaction itself
                            let Reaction {
                                emoji,
                                user_id,
                                guild_id,
                                ..
                            } = reaction;

                            // We want to make sure we have a `guild_id` associated with the reaction change, so we can
                            // later retrieve roles and assign them to members
                            if let Some(guild_id) = guild_id {
                                // We need to make sure we have a `user_id` so that we can get the `Member` associated
                                // with that id
                                if let Some(user_id) = user_id {
                                    // Use the `emoji` Unicode for comparison
                                    match emoji {
                                        // We don't support custom server emoji's because I am lazy
                                        ReactionType::Unicode(emoji) => {
                                            // Parse the role list by taking any text the reactions are attached to
                                            // then get the last "\n\n" split.
                                            let role_list = match msg.content.split("\n\n").last() {
                                                Some(role_list) => role_list,
                                                None => {
                                                    println!("Error while parsing the pinned message, couldn't find role list");
                                                    return;
                                                }
                                            };

                                            let mut role = "";

                                            // Parse the `role_list` to get the specific role by iterating over each line
                                            // comparing to see if the unicode matches, then spliting on `:`, trim the last
                                            // split, which we assume is our role name
                                            role_list.lines().filter(|line| line.contains(&emoji)).for_each(|line| {
                                                role = match line.splitn(2, ":").last() {
                                                    Some(role_split) => role_split.trim(),
                                                    None => {
                                                        println!("Error while parsing the pinned message, couldn't find role within {}", line);
                                                        return
                                                    }
                                                };
                                            });

                                            // We get the `Member` of the `GuildId` so that we can assign or remove the role
                                            let mut member = guild_id
                                                .member(ctx.http.clone(), user_id)
                                                .await
                                                .unwrap();

                                            // We need to get the `RoleId` by turning our `GuildId` into a `PartialGuild`
                                            // such that we can call `role_by_name` just so that we don't need to track
                                            // the rediculous ID's associated with each role
                                            let role_value = match guild_id
                                                .to_partial_guild(ctx.http.clone())
                                                .await
                                            {
                                                Ok(pg) => {
                                                    if let Some(role) = pg.role_by_name(role) {
                                                        role.id
                                                    } else {
                                                        println!(
                                                            "Error while getting role by name"
                                                        );
                                                        return;
                                                    }
                                                }
                                                Err(e) => {
                                                    println!(
                                                        "Error {} while getting PartialGuild",
                                                        e
                                                    );
                                                    return;
                                                }
                                            };

                                            // Finally, we take the associated action with that Role, adding it
                                            // or removing it from the particular member
                                            match action {
                                                Action::AddRole => {
                                                    println!(
                                                        "AddRole for member : {:?}",
                                                        member
                                                            .add_role(ctx.http.clone(), role_value)
                                                            .await
                                                    );
                                                }
                                                Action::RemoveRole => {
                                                    println!(
                                                        "RemoveRole for member : {:?}",
                                                        member
                                                            .remove_role(ctx.http.clone(), role_value)
                                                            .await
                                                    )
                                                }
                                            }
                                        }
                                        _ => {
                                            println!("Server custom emoji's are not supported");
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                _ => {}
            }
        }
    }
}

#[async_trait]
impl EventHandler for Handler {
    async fn reaction_add(&self, ctx: Context, add_reaction: Reaction) {
        self.handle_reaction(ctx, add_reaction, Action::AddRole)
            .await
    }

    async fn reaction_remove(&self, ctx: Context, removed_reaction: Reaction) {
        self.handle_reaction(ctx, removed_reaction, Action::RemoveRole)
            .await
    }

    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

#[tokio::main]
pub async fn main() {
    // Configure the client with your Discord bot token in the environment.
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");

    // Build our client.
    let mut client = Client::builder(token)
        .event_handler(Handler {
            cache: Mutex::new(None),
        })
        .await
        .expect("Error creating client");

    // Finally, start a single shard, and start listening to events.
    //
    // Shards will automatically attempt to reconnect, and will perform
    // exponential backoff until it reconnects.
    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}
