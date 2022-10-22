use chrono::{NaiveDateTime, Utc};
use flows_connector_dsi::discord::*;
#[allow(unused_imports)]
use wasmedge_bindgen::*;
use wasmedge_bindgen_macro::*;

fn detail(item: (i64, &str, &str, &str)) -> String {
    format!(
        "Next session for Cloud Native Wasm Day NA is \n{}.\n\nStart Time: {}\nLocation: {}\nLearn more: {}",
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
        1666616400,
        "Welcome + Opening Remarks - Kate Goldenring, Senior Software Engineer, Fermyon Technologies, Inc. ",
        "Room 310 A",
        "https://cloudnativewasmdayna22.sched.com/event/1AUD4",
    ),
    (
        1666617300,
        "Keynote: The Path to Components - Luke Wagner, Distinguished Engineer, 
Fastly ",
        "Room 310 A",
        "https://cloudnativewasmdayna22.sched.com/event/1AUD7"
    ),
    (
        1666619400,
        "Keynote: WebAssembly Development is Easy - Matt Butcher, CEO & Radu Matei, 
CTO, Fermyon Technologies, Inc.",
        "Room 310 A",
        "https://cloudnativewasmdayna22.sched.com/event/1AUDA"
    ),
    (
        1666620000
        "Keynote: Wasm: A Revolution for Browsers, Containers, and the Cloud - 
Justin Cormack, Chief Technology Officer, Docker",
        "Room 310 A",
        "https://cloudnativewasmdayna22.sched.com/event/1AUDD/"
    ),
    (
        1666620600,
        "The JVM Meets WASI: Writing Cloud-Friendly Wasm Apps Using Java and Friends 
- Joel Dice, Fermyon Technologies, Inc.",
        "Room 310 A",
        "https://cloudnativewasmdayna22.sched.com/event/1AUDG/"
    ),
    (
        1666622400,
        "Coffee Break",
        "Room 310 A",
        "https://cloudnativewasmdayna22.sched.com/event/1AUDJ/"
    ),
    (
        1666623000,
        "Porting Python to WebAssembly - Christian Heimes, Red Hat ",
        "Room 310 A",
        "https://cloudnativewasmdayna22.sched.com/event/1AUDP"
    ),
    (
        1666625100,
        "Build, Share, Run WebAssembly Apps Using the Docker Toolchain - Chris 
        Crone, Docker & Michael Yuan, Second State ",
        "Room 310 A",
        "https://cloudnativewasmdayna22.sched.com/event/1AUDh/"
    ),
    (
        1666627200,
        "⚡ Lightning Talk: SIG-Registries and Standardizing Package Management in 
        WebAssembly - Bailey Hayes, Cosmonic and Kyle Brown, SingleStore",
        "Room 310 A",
        "https://cloudnativewasmdayna22.sched.com/event/1AUDe/"
    ),
    (
        1666628100,
        "⚡ Lightning Talk: Revolutionizing Application Architecture with Wasm 
Powered Database Extensibility - Carl Sverre, SingleStore ",
        "Room 310 A",
        "https://cloudnativewasmdayna22.sched.com/event/1AUDV"
    ),
    (
        1666628700,
        "🍲 Lunch + Networking",
        "3rd Floor Foyer",
        "https://cloudnativewasmdayna22.sched.com/event/1AUDY/lunch-networking"
    ),
    (
        1666632300,
        "⚡ Lightning Talk: Implementing Wasm Debugging with .net in vscode - Thays 
Tagliaferri de Grazia, Microsoft",
        "Room 310 A",
        "https://cloudnativewasmdayna22.sched.com/event/1AUDS"
    ),
    (
        1666633200,
        "⚡ Lightning Talk: Wildly Distributed Programming: Wasm and the Future of 
Distributed Computing - Brooks Townsend, Cosmonic",
        "Room 310 A",
        "https://cloudnativewasmdayna22.sched.com/event/1AUDb/"
    ),
    (
        1666634100,
        "C# and Wasm Interface Types: Hands Across the C - Ivan Towlson, Fermyon ",
        "Room 310 A",
        "https://cloudnativewasmdayna22.sched.com/event/1AUDM/"
    ),
    (
        1666636200,
        "Mod_Wasm: Bringing WebAssembly to Apache - Daniel Lopez Ridruejo & Rafael 
Fernandez Lopez, VMware",
        "Room 310 A",
        "https://cloudnativewasmdayna22.sched.com/event/1AUDk/"
    ),
    (
        1666638000,
        "☕ Coffee Break + Networking",
        "3rd Floor Foyer",
        "https://cloudnativewasmdayna22.sched.com/event/1AUDn/"
    ),
    (
        1666638600,
        "Wasm Components: The Interchangeable Parts of Software - Taylor Thomas, 
Cosmonic",
        "Room 310 A",
        "https://cloudnativewasmdayna22.sched.com/event/1AUDq/"
    ),
    (
        1666640700,
        "Bring Your Own Bytecode to the Logging Party - Guba Sándor & Dudas Adam, 
Cisco",
        "Room 310 A",
        "https://cloudnativewasmdayna22.sched.com/event/1AUDt/"
    ),
    (
        1666642800,
        "Panel Discussion: WebAssembly - Outside In - Moderated by Bailey Hayes, 
Cosmonic; Renee Shah, Amplify, Alex Williams, Newstack & Michael Azoff, 
Omdia",
        "Room 310 A",
        "https://cloudnativewasmdayna22.sched.com/event/1AUDw"
    ),
    (
        1666645500,
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
