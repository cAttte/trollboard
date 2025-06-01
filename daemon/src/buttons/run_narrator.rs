use macros::button;
use maplit::hashmap;
use once_cell::sync::Lazy;
use registry::{Data, Hive, Security};
use std::{collections::HashMap, sync::Mutex};

static REGISTRY_SNAPSHOT: Lazy<Mutex<Record<Record<u32>>>> =
    Lazy::new(|| Mutex::new(troll_registry()));

// TODO: still very finicky. fix
#[button(desc = "Activate Narrator")]
fn run_narrator(is_press: bool) -> Result<(), &'static str> {
    let rw = Security::Read | Security::Write;
    let narr = Hive::CurrentUser
        .open("Software\\Microsoft\\Narrator", rw)
        .unwrap();
    let mut snapshot = REGISTRY_SNAPSHOT.lock().unwrap();

    crate::sudo!("taskkill", "-f -im narrator.exe");

    for (path, submap) in troll_registry() {
        let subreg = narr.open(path, rw).unwrap();
        let subsnap = snapshot.get_mut(path).unwrap();

        for (subkey, subval) in submap {
            if is_press {
                let subsnapval = subreg.value(subkey).unwrap();
                if let Data::U32(val) = subsnapval {
                    subsnap.insert(subkey, val);
                }

                println!("setting {}/{} = {}", path, subkey, subval);
                subreg.set_value(subkey, &Data::U32(subval)).unwrap();
            } else {
                let subsnapval = subsnap.get(subkey).unwrap();
                let _ = subreg.set_value(subkey, &Data::U32(*subsnapval));
            }
        }

        // TODO: CHECK IF WAS ALREADY RUNNING
        if is_press {
            crate::sudo!("narrator");
        } else {
            crate::sudo!("taskkill", "-f -im narrator.exe");
        }
    }

    Ok(())
}

type Record<T> = HashMap<&'static str, T>;
pub fn troll_registry() -> Record<Record<u32>> {
    hashmap! {
        "" => hashmap! {
            "EchoWords" => 1,
            "EchoChars" => 1,
            "SpeechSpeed" => 20,
            "SpeechPitch" => 20
        },
        "NoRoam" => hashmap! {
            "DuckAudio" => 1,
            "UserVerbosityLevel" => 5,
            "ContextVerbosityLevelV2" => 5,
            "EchoFunctionKeys" => 1,
            "EchoModifierKeys" => 1,
            "EchoNavigationKeys" => 1,
            "EchoToggleKeys" => 1,
            "SpeechVolume" => 100,
            "NarratorModifiers" => 2 // Insert
        },
        "NarratorHome" => hashmap! {
            "AutoStart" => 0
        }
    }
}
