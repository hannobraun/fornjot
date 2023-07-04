use std::fmt::Write;

use tokio::{fs::File, io::AsyncWriteExt};

use super::util;

pub async fn create_sponsor_update() -> anyhow::Result<()> {
    let month = util::now_ym();
    let date = util::now_ymd();

    let mut file = util::create_blog_post_file("sponsors", &month).await?;
    generate_update(date, month, &mut file).await?;

    Ok(())
}

async fn generate_update(
    date: String,
    month: String,
    file: &mut File,
) -> anyhow::Result<()> {
    let mut buf = String::new();
    write!(
        buf,
        "\
+++
title = \"Sponsor Update - {month}\"
date = {date}

# Uncomment to generate the HTML for the email newsletter.
# template = \"newsletter/email.html\"
+++

Hey folks!

I just sent out the new sponsor update! Topics this month include:

- **TASK: Summarize sponsor update.**

If you want to receive monthly behind-the-scenes updates too, why not support Fornjot by [becoming a sponsor](https://github.com/sponsors/hannobraun)? You can start with as little as $2 a month. More substantial contributions are also welcome, of course 😁

I dedicate a substantial chunk of my week to working on Fornjot. Your contribution can help make that more sustainable.


### Not receiving these updates?

I've been sending out an update every month since February 2022. If you are a sponsor and haven't received those updates, maybe you are not opted in? Update your sponsorship over at GitHub, and make sure you check `Receive email updates from hannobraun`. Also make sure to check the spam folder in your email client.

If you still haven't received an update, [please contact me](mailto:hanno@braun-odw.eu). I'm happy to send you a copy directly.

I'm sorry for any inconvenience! Unfortunately, GitHub gives me no control over, or insight into, who is receiving those updates.
"
    )?;

    file.write_all(buf.as_bytes()).await?;

    Ok(())
}
