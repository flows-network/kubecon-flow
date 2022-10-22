use chrono::{NaiveDateTime, Utc};
use flows_connector_dsi::discord::*;
#[allow(unused_imports)]
use wasmedge_bindgen::*;
use wasmedge_bindgen_macro::*;

fn detail(item: (i64, &str, &str, &str)) -> String {
    format!(
        "Next session is \n{}.\n\nTime: {}\nLocation: {}\nLink: {}",
        item.1,
        NaiveDateTime::from_timestamp(item.0 as i64, 0).to_string(),
        item.2,
        item.3,
    )
}

#[wasmedge_bindgen]
pub fn run(s: String) -> Result<String, String> {
    let message = inbound(s)?;

    if message.content != "/wasm" {
        return Ok(String::new());
    }

    let now = Utc::now().timestamp();

    let info = [(
        1666623609,
        "Welcome + Opening Remarks - Kate Goldenring, Senior Software Engineer, Fermyon Technologies, Inc. ",
        "Room 310 A",
        "https://cloudnativewasmdayna22.sched.com/event/1AUD4?iframe=no",
    ),
    (
        1666624500,
        "Keynote: The Path to Components - Luke Wagner, Distinguished Engineer, 
Fastly ",
        "Room 310 A",
        "https://sched.co/1AUD7"
    ),
    (
        1666626600,
        "Keynote: WebAssembly Development is Easy - Matt Butcher, CEO & Radu Matei, 
CTO, Fermyon Technologies, Inc.",
        "Room 310 A",
        "https://sched.co/1AUDA"
    ),
    (
        1666627200,
        "Keynote: Wasm: A Revolution for Browsers, Containers, and the Cloud - 
Justin Cormack, Chief Technology Officer, Docker",
        "Room 310 A",
        "https://cloudnativewasmdayna22.sched.com/event/1AUDD/"
    ),
    (
        1666627800,
        "The JVM Meets WASI: Writing Cloud-Friendly Wasm Apps Using Java and Friends 
- Joel Dice, Fermyon Technologies, Inc.",
        "Room 310 A",
        "https://cloudnativewasmdayna22.sched.com/event/1AUDG/"
    ),
    (
        1666629600,
        "Coffee Break",
        "Room 310 A",
        "https://cloudnativewasmdayna22.sched.com/event/1AUDJ/"
    ),
    (
        1666630200,
        "Porting Python to WebAssembly - Christian Heimes, Red Hat ",
        "Room 310 A",
        "https://cloudnativewasmdayna22.sched.com/event/1AUDP"
    ),
    (
        1666632300,
        "Build, Share, Run WebAssembly Apps Using the Docker Toolchain - Chris 
        Crone, Docker & Michael Yuan, Second State ",
        "Room 310 A",
        "https://cloudnativewasmdayna22.sched.com/event/1AUDh/"
    ),
    (
        1666634400,
        "⚡ Lightning Talk: SIG-Registries and Standardizing Package Management in 
        WebAssembly - Bailey Hayes, Cosmonic and Kyle Brown, SingleStore",
        "Room 310 A",
        "https://cloudnativewasmdayna22.sched.com/event/1AUDe/"
    ),
    (
        1666635300,
        "⚡ Lightning Talk: Revolutionizing Application Architecture with Wasm 
Powered Database Extensibility - Carl Sverre, SingleStore ",
        "Room 310 A",
        "https://cloudnativewasmdayna22.sched.com/event/1AUDV"
    ),
    (
        1666635900,
        "🍲 Lunch + Networking",
        "3rd Floor Foyer",
        "https://cloudnativewasmdayna22.sched.com/event/1AUDY/lunch-networking"
    ),
    (
        1666639500,
        "⚡ Lightning Talk: Implementing Wasm Debugging with .net in vscode - Thays 
Tagliaferri de Grazia, Microsoft",
        "Room 310 A",
        "https://cloudnativewasmdayna22.sched.com/event/1AUDS"
    ),
    (
        1666640400,
        "⚡ Lightning Talk: Wildly Distributed Programming: Wasm and the Future of 
Distributed Computing - Brooks Townsend, Cosmonic",
        "Room 310 A",
        "https://cloudnativewasmdayna22.sched.com/event/1AUDb/"
    ),
    (
        1666641300,
        "C# and Wasm Interface Types: Hands Across the C - Ivan Towlson, Fermyon ",
        "Room 310 A",
        "https://cloudnativewasmdayna22.sched.com/event/1AUDM/"
    ),
    (
        1666643400,
        "Mod_Wasm: Bringing WebAssembly to Apache - Daniel Lopez Ridruejo & Rafael 
Fernandez Lopez, VMware",
        "Room 310 A",
        "https://cloudnativewasmdayna22.sched.com/event/1AUDk/"
    ),
    (
        1666645200,
        "☕ Coffee Break + Networking",
        "3rd Floor Foyer",
        "https://cloudnativewasmdayna22.sched.com/event/1AUDn/"
    ),
    (
        1666645800,
        "Wasm Components: The Interchangeable Parts of Software - Taylor Thomas, 
Cosmonic",
        "Room 310 A",
        "https://cloudnativewasmdayna22.sched.com/event/1AUDq/"
    ),
    (
        1666647900,
        "Bring Your Own Bytecode to the Logging Party - Guba Sándor & Dudas Adam, 
Cisco",
        "Room 310 A",
        "https://cloudnativewasmdayna22.sched.com/event/1AUDt/"
    ),
    (
        1666650000,
        "Panel Discussion: WebAssembly - Outside In - Moderated by Bailey Hayes, 
Cosmonic; Renee Shah, Amplify, Alex Williams, Newstack & Michael Azoff, 
Omdia",
        "Room 310 A",
        "https://cloudnativewasmdayna22.sched.com/event/1AUDw"
    ),
    (
        1666652700,
        "Closing Remarks - Connor Hicks, Founder & CEO, Suborbital ",
        "Room 310 A",
        "https://cloudnativewasmdayna22.sched.com/event/1AUDz/"
    )]
    .into_iter()
    .find(|item| item.0 > now)
    .map(|item| detail(item))
    .unwrap_or("Kubecon is over.".to_string());

    outbound::say(info, Some(message)).build()
}
