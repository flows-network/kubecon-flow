use chrono::{Utc, TimeZone, DateTime};
use flows_connector_dsi::discord::*;
#[allow(unused_imports)]
use wasmedge_bindgen::*;
use wasmedge_bindgen_macro::*;

fn detail(item: (DateTime<Utc>, DateTime<Utc>, &str, &str, &str)) -> String {
    format!(
        "Next session for Cloud Native Wasm Day NA is \n{}.\n\nTime: {} - {}\nLocation: {}\nLearn more: {}",
        item.2,
        item.0,
        item.1,
        item.3,
        item.4,
    )
}

#[wasmedge_bindgen]
pub fn run(s: String) -> Result<String, String> {
    let message = inbound(s)?;

    if message.content != "/w" && message.content != "/wasm" {
        return Ok(String::new());
    }

    let now = Utc::now();
    let date = Utc.ymd(2022, 10, 24);

    let info = [(
        date.and_hms(9, 0, 0),
        date.and_hms(9, 10, 0),
        "Welcome + Opening Remarks - Kate Goldenring, Senior Software Engineer, Fermyon Technologies, Inc. ",
        "Room 310 A",
        "https://cloudnativewasmdayna22.sched.com/event/1AUD4",
    ),
    (
        date.and_hms(9, 15, 0),
        date.and_hms(9, 45, 0),
        "Keynote: The Path to Components - Luke Wagner, Distinguished Engineer, 
Fastly ",
        "Room 310 A",
        "https://cloudnativewasmdayna22.sched.com/event/1AUD7"
    ),
    (
        date.and_hms(9, 50, 0),
        date.and_hms(9, 55, 0),
        "Keynote: WebAssembly Development is Easy - Matt Butcher, CEO & Radu Matei, 
CTO, Fermyon Technologies, Inc.",
        "Room 310 A",
        "https://cloudnativewasmdayna22.sched.com/event/1AUDA"
    ),
    (
        date.and_hms(10, 0, 0),
        date.and_hms(10, 5, 0),
        "Keynote: Wasm: A Revolution for Browsers, Containers, and the Cloud - 
Justin Cormack, Chief Technology Officer, Docker",
        "Room 310 A",
        "https://cloudnativewasmdayna22.sched.com/event/1AUDD/"
    ),
    (
        date.and_hms(10, 10, 0),
        date.and_hms(10, 40, 0),
        "The JVM Meets WASI: Writing Cloud-Friendly Wasm Apps Using Java and Friends 
- Joel Dice, Fermyon Technologies, Inc.",
        "Room 310 A",
        "https://cloudnativewasmdayna22.sched.com/event/1AUDG/"
    ),
    (
        date.and_hms(10, 40, 0),
        date.and_hms(10, 50, 0),
        "Coffee Break",
        "Room 310 A",
        "https://cloudnativewasmdayna22.sched.com/event/1AUDJ/"
    ),
    (
        date.and_hms(10, 50, 0),
        date.and_hms(11, 20, 0),
        "Porting Python to WebAssembly - Christian Heimes, Red Hat ",
        "Room 310 A",
        "https://cloudnativewasmdayna22.sched.com/event/1AUDP"
    ),
    (
        date.and_hms(11, 25, 0),
        date.and_hms(11, 55, 0),
        "Build, Share, Run WebAssembly Apps Using the Docker Toolchain - Chris 
        Crone, Docker & Michael Yuan, Second State ",
        "Room 310 A",
        "https://cloudnativewasmdayna22.sched.com/event/1AUDh/"
    ),
    (
        date.and_hms(12, 0, 0),
        date.and_hms(12, 10, 0),
        "⚡ Lightning Talk: SIG-Registries and Standardizing Package Management in 
        WebAssembly - Bailey Hayes, Cosmonic and Kyle Brown, SingleStore",
        "Room 310 A",
        "https://cloudnativewasmdayna22.sched.com/event/1AUDe/"
    ),
    (
        date.and_hms(12, 15, 0),
        date.and_hms(12, 25, 0),
        "⚡ Lightning Talk: Revolutionizing Application Architecture with Wasm 
Powered Database Extensibility - Carl Sverre, SingleStore ",
        "Room 310 A",
        "https://cloudnativewasmdayna22.sched.com/event/1AUDV"
    ),
    (
        date.and_hms(12, 25, 0),
        date.and_hms(13, 25, 0),
        "🍲 Lunch + Networking",
        "3rd Floor Foyer",
        "https://cloudnativewasmdayna22.sched.com/event/1AUDY/lunch-networking"
    ),
    (
        date.and_hms(13, 25, 0),
        date.and_hms(13, 35, 0),
        "⚡ Lightning Talk: Implementing Wasm Debugging with .net in vscode - Thays 
Tagliaferri de Grazia, Microsoft",
        "Room 310 A",
        "https://cloudnativewasmdayna22.sched.com/event/1AUDS"
    ),
    (
        date.and_hms(13, 40, 0),
        date.and_hms(13, 50, 0),
        "⚡ Lightning Talk: Wildly Distributed Programming: Wasm and the Future of 
Distributed Computing - Brooks Townsend, Cosmonic",
        "Room 310 A",
        "https://cloudnativewasmdayna22.sched.com/event/1AUDb/"
    ),
    (
        date.and_hms(13, 55, 0),
        date.and_hms(14, 25, 0),
        "C# and Wasm Interface Types: Hands Across the C - Ivan Towlson, Fermyon ",
        "Room 310 A",
        "https://cloudnativewasmdayna22.sched.com/event/1AUDM/"
    ),
    (
        date.and_hms(14, 30, 0),
        date.and_hms(15, 0, 0),
        "Mod_Wasm: Bringing WebAssembly to Apache - Daniel Lopez Ridruejo & Rafael 
Fernandez Lopez, VMware",
        "Room 310 A",
        "https://cloudnativewasmdayna22.sched.com/event/1AUDk/"
    ),
    (
        date.and_hms(15, 0, 0),
        date.and_hms(15, 10, 0),
        "☕ Coffee Break + Networking",
        "3rd Floor Foyer",
        "https://cloudnativewasmdayna22.sched.com/event/1AUDn/"
    ),
    (
        date.and_hms(15, 10, 0),
        date.and_hms(15, 40, 0),
        "Wasm Components: The Interchangeable Parts of Software - Taylor Thomas, 
Cosmonic",
        "Room 310 A",
        "https://cloudnativewasmdayna22.sched.com/event/1AUDq/"
    ),
    (
        date.and_hms(15, 45, 0),
        date.and_hms(16, 15, 0),
        "Bring Your Own Bytecode to the Logging Party - Guba Sándor & Dudas Adam, 
Cisco",
        "Room 310 A",
        "https://cloudnativewasmdayna22.sched.com/event/1AUDt/"
    ),
    (
        date.and_hms(16, 20, 0),
        date.and_hms(17, 0, 0),
        "Panel Discussion: WebAssembly - Outside In - Moderated by Bailey Hayes, 
Cosmonic; Renee Shah, Amplify, Alex Williams, Newstack & Michael Azoff, 
Omdia",
        "Room 310 A",
        "https://cloudnativewasmdayna22.sched.com/event/1AUDw"
    ),
    (
        date.and_hms(17, 5, 0),
        date.and_hms(17, 10, 0),
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
