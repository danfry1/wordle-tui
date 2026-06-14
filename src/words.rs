//! Embedded five-letter word lists.
//!
//! `ANSWERS` is the pool the daily/secret word is drawn from. `EXTRA_VALID`
//! contains additional words that are accepted as guesses but never chosen as
//! answers. A guess is valid if it appears in either list. Every entry in both
//! lists is exactly five lowercase ASCII letters.

/// Words eligible to be the secret answer (common, well-known words).
pub const ANSWERS: &[&str] = &[
    "about", "above", "abuse", "actor", "acute", "admit", "adopt", "adult", "after", "again",
    "agent", "agree", "ahead", "alarm", "album", "alert", "alike", "alive", "allow", "alone",
    "along", "alter", "among", "anger", "angle", "angry", "apart", "apple", "apply", "arena",
    "argue", "arise", "array", "aside", "asset", "audio", "audit", "avoid", "award", "aware",
    "badly", "baker", "bases", "basic", "basis", "beach", "began", "begin", "begun", "being",
    "below", "bench", "billy", "birth", "black", "blame", "blind", "block", "blood", "board",
    "boost", "booth", "bound", "brain", "brand", "brave", "bread", "break", "breed", "brief",
    "bring", "broad", "broke", "brown", "build", "built", "buyer", "cable", "calif", "carry",
    "catch", "cause", "chain", "chair", "chaos", "charm", "chart", "chase", "cheap", "check",
    "chest", "chief", "child", "china", "chose", "civil", "claim", "class", "clean", "clear",
    "click", "climb", "clock", "close", "cloud", "coach", "coast", "could", "count", "court",
    "cover", "craft", "crash", "crazy", "cream", "crime", "cross", "crowd", "crown", "crude",
    "curve", "cycle", "daily", "dance", "dated", "dealt", "death", "debut", "delay", "depth",
    "doing", "doubt", "dozen", "draft", "drama", "drank", "drawn", "dream", "dress", "drill",
    "drink", "drive", "drove", "dying", "eager", "early", "earth", "eight", "elite", "empty",
    "enemy", "enjoy", "enter", "entry", "equal", "error", "event", "every", "exact", "exist",
    "extra", "faith", "false", "fault", "fiber", "field", "fifth", "fifty", "fight", "final",
    "first", "fixed", "flash", "fleet", "floor", "fluid", "focus", "force", "forth", "forty",
    "forum", "found", "frame", "frank", "fraud", "fresh", "front", "fruit", "fully", "funny",
    "giant", "given", "glass", "globe", "going", "grace", "grade", "grand", "grant", "grass",
    "great", "green", "gross", "group", "grown", "guard", "guess", "guest", "guide", "happy",
    "harsh", "heart", "heavy", "hence", "henry", "horse", "hotel", "house", "human", "ideal",
    "image", "index", "inner", "input", "issue", "japan", "jimmy", "joint", "jones", "judge",
    "known", "label", "large", "laser", "later", "laugh", "layer", "learn", "lease", "least",
    "leave", "legal", "level", "lewis", "light", "limit", "links", "lives", "local", "logic",
    "loose", "lower", "lucky", "lunch", "lying", "magic", "major", "maker", "march", "maria",
    "match", "maybe", "mayor", "meant", "media", "metal", "might", "minor", "minus", "mixed",
    "model", "money", "month", "moral", "motor", "mount", "mouse", "mouth", "moved", "movie",
    "music", "needs", "never", "newly", "night", "noise", "north", "noted", "novel", "nurse",
    "occur", "ocean", "offer", "often", "order", "other", "ought", "paint", "panel", "paper",
    "party", "peace", "peter", "phase", "phone", "photo", "piece", "pilot", "pitch", "place",
    "plain", "plane", "plant", "plate", "point", "pound", "power", "press", "price", "pride",
    "prime", "print", "prior", "prize", "proof", "proud", "prove", "queen", "quick", "quiet",
    "quite", "radio", "raise", "range", "rapid", "ratio", "reach", "ready", "realm", "rebel",
    "refer", "right", "rigid", "rival", "river", "robin", "roger", "roman", "rough", "round",
    "route", "royal", "rural", "scale", "scene", "scope", "score", "sense", "serve", "seven",
    "shall", "shape", "share", "sharp", "sheet", "shelf", "shell", "shift", "shine", "shirt",
    "shock", "shoot", "short", "shown", "sight", "since", "sixth", "sixty", "sized", "skill",
    "sleep", "slide", "small", "smart", "smile", "smith", "smoke", "snake", "solid", "solve",
    "sorry", "sound", "south", "space", "spare", "speak", "speed", "spend", "spent", "split",
    "spoke", "sport", "staff", "stage", "stake", "stand", "start", "state", "steam", "steel",
    "steep", "steer", "stick", "still", "stock", "stone", "stood", "store", "storm", "story",
    "strip", "stuck", "study", "stuff", "style", "sugar", "suite", "super", "sweet", "table",
    "taken", "taste", "taxes", "teach", "teeth", "terry", "texas", "thank", "theft", "their",
    "theme", "there", "these", "thick", "thing", "think", "third", "those", "three", "threw",
    "throw", "tight", "times", "tired", "title", "today", "topic", "total", "touch", "tough",
    "tower", "track", "trade", "train", "treat", "trend", "trial", "tribe", "trick", "tried",
    "tries", "truck", "truly", "trust", "truth", "twice", "uncle", "under", "undue", "union",
    "unity", "until", "upper", "upset", "urban", "usage", "usual", "valid", "value", "video",
    "virus", "visit", "vital", "voice", "waste", "watch", "water", "wheel", "where", "which",
    "while", "white", "whole", "whose", "woman", "world", "worry", "worse", "worst", "worth",
    "would", "wound", "write", "wrong", "wrote", "yield", "young", "youth",
];

/// Extra words accepted as guesses but never chosen as the answer.
pub const EXTRA_VALID: &[&str] = &[
    "aback", "abide", "abode", "abort", "ached", "acids", "acorn", "adept", "adobe", "adore",
    "afoot", "agile", "aglow", "aided", "aimed", "aisle", "alder", "alias", "amber", "amend",
    "amiss", "ample", "amply", "angel", "ankle", "annoy", "antic", "anvil", "aorta", "apron",
    "areas", "armor", "aroma", "arrow", "ascot", "atlas", "atoll", "atone", "attic", "augur",
    "aught", "aunts", "aural", "avert", "axiom", "azure", "bacon", "badge", "bagel", "baron",
    "basil", "baste", "bayou", "beads", "beard", "beast", "beech", "befit", "berry", "bezel",
    "bigot", "binge", "bison", "blade", "blast", "blaze", "bleak", "bleed", "blend", "bless",
    "blimp", "blink", "bliss", "bloat", "blond", "blown", "bluff", "blunt", "blurb", "blurt",
    "blush", "boast", "bogus", "bolts", "bonus", "booze", "borne", "bough", "boxer", "bract",
    "braid", "brake", "brash", "brawl", "brawn", "bribe", "brine", "briny", "brisk", "broth",
    "brunt", "brush", "brute", "buddy", "budge", "buggy", "bugle", "bulge", "bulky", "bunch",
    "bunny", "burly", "burnt", "burst", "cabin", "cacao", "cache", "caddy", "cadet", "cagey",
    "camel", "candy", "canny", "canoe", "caper", "carat", "cargo", "carol", "caulk", "cease",
    "cedar", "chafe", "chalk", "champ", "chant", "chard", "cheek", "cheer", "chess", "chewy",
    "chili", "chill", "chime", "chirp", "chock", "choir", "choke", "chomp", "chord", "chore",
    "chuck", "chump", "chunk", "churn", "chute", "cider", "cigar", "cinch", "circa", "civic",
    "clack", "clamp", "clang", "clank", "clash", "clasp", "cleat", "cleft", "clerk", "cliff",
    "cling", "clink", "cloak", "clone", "clout", "clove", "clown", "cluck", "clued", "clump",
    "clung", "cobra", "cocoa", "colon", "comet", "comfy", "comic", "comma", "conch", "condo",
    "coral", "corny", "couch", "cough", "coupe", "covet", "cower", "crack", "cramp", "crane",
    "crank", "crate", "crave", "craze", "creak", "credo", "creed", "creek", "creep", "creme",
    "crepe", "crept", "cress", "crest", "crick", "crisp", "croak", "crock", "crone", "crony",
    "crook", "croon", "crumb", "crush", "crust", "crypt", "cubic", "cumin", "curio", "curly",
    "curry", "curse", "curvy", "cyber", "cynic", "daddy", "dairy", "daisy", "dandy", "datum",
    "daunt", "debit", "debug", "decal", "decay", "decor", "decoy", "decry", "defer", "deign",
    "deity", "delta", "delve", "demon", "demur", "denim", "dense", "depot", "deter", "devil",
    "diary", "dicey", "digit", "dimly", "diner", "dingo", "dingy", "dirge", "dirty", "disco",
    "ditch", "ditto", "ditty", "diver", "dizzy", "dodge", "dodgy", "dogma", "donor", "donut",
    "dopey", "dough", "dowdy", "downy", "dowry", "drape", "drawl", "dread", "drier", "drift",
    "droit", "droll", "drone", "drool", "droop", "dross", "drunk", "dryly", "ducat", "ducky",
    "duchy", "dully", "dummy", "dumpy", "dunce", "dusky", "dusty", "dutch", "dwarf", "dwell",
    "dwelt", "eagle", "easel", "eaten", "eater", "ebony", "eclat", "edged", "edict", "edify",
    "eerie", "egret", "elate", "elbow", "elder", "elect", "elegy", "elfin", "elude", "elves",
    "ember", "embed", "emcee", "endow", "epoch", "epoxy", "equip", "erase", "erect", "erode",
    "essay", "ester", "ether", "ethic", "ethos", "evade", "evict", "evoke", "exalt", "excel",
    "exert", "exile", "expel", "extol", "exult", "fable", "facet", "faint", "fairy", "fancy",
    "farce", "fatal", "fatty", "feast", "fecal", "feign", "femur", "fence", "feral", "ferry",
    "fetal", "fetch", "fetid", "fetus", "fever", "fewer", "fiend", "fiery", "filer", "filet",
    "filly", "filmy", "filth", "finch", "finer", "fishy", "fixer", "fizzy", "fjord", "flack",
    "flail", "flair", "flake", "flaky", "flank", "flare", "flask", "fleck", "flesh", "flick",
    "flier", "fling", "flint", "flirt", "float", "flock", "flora", "floss", "flout", "flown",
    "fluff", "flung", "flush", "flute", "flyer", "foamy", "foggy", "foist", "folly", "foray",
    "forge", "forgo", "forte", "fount", "foyer", "freak", "freed", "freer", "frill", "frisk",
    "fritz", "frock", "frond", "frost", "froth", "frown", "froze", "fryer", "fudge", "fugue",
    "fungi", "funky", "furor", "furry", "fussy", "fuzzy", "gaily", "gamma", "gamut", "gassy",
    "gaudy", "gauge", "gaunt", "gauze", "gavel", "gawky", "gayly", "gecko", "geeky", "genie",
    "genre", "ghost", "ghoul", "giddy", "girth", "giver", "gizmo", "glade", "gland", "glare",
    "glaze", "gleam", "glean", "glide", "glint", "gloat", "gloom", "glory", "gloss", "glove",
    "glyph", "gnash", "gnome", "godly", "golem", "goner", "gonna", "goody", "gooey", "goofy",
    "goose", "gorge", "gouge", "gourd", "grail", "grape", "graph", "grasp", "grate", "grave",
    "gravy", "graze", "greed", "greet", "grief", "grill", "grime", "grimy", "grind", "gripe",
    "groan", "groin", "groom", "grope", "grout", "grove", "growl", "gruel", "gruff", "grunt",
    "guava", "guild", "guile", "guilt", "guise", "gully", "gumbo", "gummy", "guppy", "gusto",
    "gusty", "gutsy", "habit", "hairy", "halve", "handy", "hardy", "harem", "harpy", "hasty",
    "hatch", "haunt", "haven", "havoc", "hazel", "heady", "heath", "hedge", "hefty", "heist",
    "helix", "hello", "herbs", "heron", "hilly", "hinge", "hippo", "hippy", "hoard", "hobby",
    "hoist", "holly", "homer", "honey", "honor", "hooch", "hound", "hovel", "hover", "howdy",
    "humid", "humor", "humph", "hunch", "hurry", "husky", "hussy", "hutch", "hydro", "hyena",
    "hymen", "icily", "icing", "idiom", "idiot", "idler", "idyll", "igloo", "iliac", "imbue",
    "impel", "imply", "inane", "inbox", "incur", "infer", "ingot", "inlay", "inlet", "inter",
    "intro", "ionic", "irate", "irony", "islet", "itchy", "ivory", "jaunt", "jazzy", "jelly",
    "jerky", "jetty", "jewel", "jiffy", "joker", "jolly", "joust", "jowls", "juice", "juicy",
    "jumbo", "jumpy", "junta", "junto", "juror", "kappa", "karma", "kayak", "kebab", "khaki",
    "kinky", "kiosk", "kitty", "knack", "knave", "knead", "kneel", "knelt", "knife", "knock",
    "knoll", "koala", "krill", "labor", "laden", "ladle", "lager", "lance", "lanky", "lapel",
    "lapse", "larva", "lasso", "latch", "lathe", "leafy", "leaky", "leant", "leapt", "ledge",
    "leech", "leery", "lefty", "legos", "lemon", "lemur", "leper", "levee", "libel", "liege",
    "lifer", "liken", "lilac", "limbo", "linen", "liner", "lingo", "lipid", "lithe", "liver",
    "livid", "llama", "loamy", "loath", "lobby", "locus", "lodge", "lofty", "lonis", "loony",
    "loopy", "loser", "louse", "lousy", "lover", "lowly", "loyal", "lucid", "lumen", "lumpy",
    "lunar", "lunge", "lupus", "lurch", "lurid", "lusty", "lyric", "macaw", "macho", "macro",
    "madam", "madly", "mafia", "maize", "mamba", "mango", "mangy", "mania", "manic", "manly",
    "manor", "maple", "marry", "marsh", "mason", "masse", "matey", "mauve", "maxim", "mealy",
    "meaty", "medal", "melon", "mercy", "merge", "merit", "merry", "messy", "meter", "metro",
    "midge", "midst", "milky", "mimic", "mince", "miner", "minim", "minty", "mirth", "miser",
    "missy", "mocha", "modal", "modem", "mogul", "moist", "molar", "moldy", "moody", "moose",
    "moron", "morph", "mossy", "motel", "motif", "motto", "moult", "mound", "mourn", "mousy",
    "mover", "mucky", "mucus", "muddy", "mulch", "mummy", "munch", "mural", "murky", "mushy",
    "musky", "musty", "myrrh", "nadir", "naive", "nanny", "nasal", "nasty", "natal", "naval",
    "navel", "needy", "neigh", "nerdy", "nerve", "nicer", "niche", "niece", "nifty", "ninja",
    "ninny", "ninth", "noble", "nobly", "nomad", "noose", "nosey", "notch", "nudge", "nutty",
    "nylon", "nymph", "oaken", "oaths", "obese", "octal", "octet", "odder", "oddly", "offal",
    "olden", "older", "olive", "ombre", "omega", "onion", "onset", "oomph", "opine", "opium",
    "optic", "orbit", "organ", "otter", "ounce", "outdo", "outer", "outgo", "ovary", "overt",
    "ovine", "ovoid", "owing", "owner", "oxide", "ozone", "paddy", "pagan", "paler", "palsy",
    "panic", "pansy", "papal", "parka", "parry", "parse", "pasta", "paste", "pasty", "patch",
    "patio", "patsy", "patty", "pause", "payee", "payer", "peach", "pearl", "pecan", "pedal",
    "penal", "perch", "peril", "perky", "pesky", "pesto", "petal", "petty", "phony", "piano",
    "picky", "piety", "piggy", "pinch", "piney", "pinky", "pinto", "pixel", "pixie", "pizza",
    "plaid", "plait", "plank", "plaza", "plead", "pleat", "plied", "plier", "pluck", "plumb",
    "plume", "plump", "plunk", "plush", "poesy", "poise", "poker", "polar", "polka", "polyp",
    "pooch", "poppy", "porch", "poser", "posit", "posse", "potty", "pouch", "pouty", "prank",
    "prawn", "preen", "prick", "pricy", "primo", "primp", "probe", "prone", "prong", "props",
    "prose", "proxy", "prude", "prune", "psalm", "pubic", "pudgy", "puffy", "pulpy", "pulse",
    "punch", "pupil", "puppy", "puree", "purer", "purge", "purse", "pushy", "putty", "pygmy",
    "quack", "quail", "quake", "qualm", "quark", "quart", "quash", "quasi", "quell", "query",
    "quest", "queue", "quill", "quirk", "quota", "quote", "rabbi", "rabid", "racer", "radar",
    "radii", "ralph", "ramen", "ranch", "randy", "raspy", "ratty", "raven", "rayon", "razor",
    "react", "rearm", "rebar", "rebus", "rebut", "recap", "recur", "recut", "regal", "rehab",
    "reign", "relax", "relay", "relic", "remit", "renal", "renew", "repay", "repel", "reply",
    "rerun", "reset", "resin", "retch", "retro", "retry", "reuse", "revel", "revue", "rhino",
    "rhyme", "ridge", "rifle", "riper", "risen", "riser", "risky", "rivet", "roach", "roast",
    "robot", "rocky", "rodeo", "roomy", "roost", "rotor", "rouge", "rouse", "rover", "rowdy",
    "rowel", "ruddy", "ruder", "rugby", "ruler", "rumba", "rumor", "runic", "runny", "rusty",
    "saber", "sadly", "safer", "saint", "salad", "sally", "salon", "salsa", "salty", "salve",
    "salvo", "sandy", "saner", "sappy", "sassy", "satin", "satyr", "sauce", "saucy", "sauna",
    "saute", "savor", "savoy", "savvy", "scald", "scalp", "scaly", "scamp", "scant", "scare",
    "scarf", "scary", "scion", "scoff", "scold", "scone", "scoop", "scoot", "scorn", "scour",
    "scout", "scowl", "scram", "scrap", "scree", "screw", "scrub", "scrum", "scuba", "sedan",
    "seedy", "segue", "seize", "semen", "sepia", "serif", "serum", "sewer", "shade", "shady",
    "shaft", "shake", "shaky", "shale", "shame", "shank", "shawl", "shear", "sheen", "sheep",
    "sheer", "sheik", "shied", "shire", "shoal", "shone", "shook", "shore", "shorn", "shout",
    "shove", "shred", "shrew", "shrub", "shrug", "shuck", "shunt", "shush", "shyly", "siege",
    "sieve", "silky", "silly", "silty", "sinew", "singe", "siren", "sissy", "skate", "skein",
    "skier", "skiff", "skimp", "skirt", "skulk", "skull", "skunk", "slack", "slain", "slang",
    "slant", "slash", "slate", "slave", "sleek", "sleet", "slept", "slice", "slick", "slime",
    "slimy", "sling", "slink", "sloop", "slope", "slosh", "sloth", "slump", "slung", "slunk",
    "slurp", "slush", "slyly", "smack", "smear", "smell", "smelt", "smirk", "smite", "smock",
    "smote", "snack", "snail", "snare", "snarl", "sneak", "sneer", "snide", "sniff", "snipe",
    "snoop", "snore", "snort", "snout", "snowy", "snuck", "snuff", "soapy", "sober", "soggy",
    "solar", "sonar", "sonic", "sooth", "sooty", "soppy", "sowed", "spade", "spank", "spasm",
    "spawn", "spear", "speck", "spell", "spelt", "sperm", "spice", "spicy", "spied", "spiel",
    "spike", "spiky", "spill", "spilt", "spine", "spiny", "spire", "spite", "splat", "spoil",
    "spoof", "spool", "spoon", "spore", "spout", "spray", "spree", "sprig", "spunk", "spurn",
    "spurt", "squad", "squat", "squib", "stack", "stain", "stair", "stale", "stalk", "stall",
    "stamp", "stank", "stark", "stash", "stave", "stead", "stean", "steed", "stein", "stern",
    "stiff", "sting", "stink", "stint", "stoic", "stoke", "stole", "stomp", "stony", "stool",
    "stoop", "stork", "stout", "stove", "strap", "straw", "stray", "strut", "stump", "stung",
    "stunk", "stunt", "suave", "sulky", "sully", "sumac", "sunny", "surer", "surge", "surly",
    "swami", "swamp", "swarm", "swash", "swath", "swear", "sweat", "swept", "swift", "swill",
    "swine", "swing", "swirl", "swish", "swoon", "swoop", "sword", "swore", "sworn", "swung",
    "synod", "syrup", "tabby", "taboo", "tacit", "tacky", "taffy", "taint", "tally", "talon",
    "tamer", "tango", "tangy", "taper", "tapir", "tardy", "tarot", "tarry", "tasty", "tatty",
    "taunt", "taupe", "tawny", "teary", "tease", "teddy", "tempo", "tenet", "tenor", "tense",
    "tenth", "tepee", "tepid", "terse", "testy", "theta", "thigh", "thong", "thorn", "throb",
    "thumb", "thump", "thyme", "tiara", "tibia", "tidal", "tiger", "tilde", "tipsy", "titan",
    "tithe", "toast", "toddy", "token", "tonal", "tonga", "tonic", "tooth", "topaz", "torch",
    "torso", "torus", "totem", "toxic", "toxin", "trace", "tract", "trait", "tramp", "trash",
    "trawl", "tread", "tweak", "tweed", "tweet", "twine", "twirl", "twist", "twixt", "tying",
    "udder", "ulcer", "ultra", "umbra", "unbar", "uncap", "unfed", "unfit", "unify", "unlit",
    "unmet", "unset", "untie", "unwed", "unzip", "upend", "urine", "usher", "usurp", "utile",
    "utter", "vague", "valet", "valor", "vapid", "vapor", "vault", "vaunt", "vegan", "venom",
    "venue", "verge", "verse", "verso", "vicar", "vigil", "vigor", "villa", "vinyl", "viola",
    "viper", "viral", "visor", "vista", "vivid", "vixen", "vocal", "vodka", "vogue", "voila",
    "voter", "vouch", "vowel", "wacky", "wafer", "wager", "wagon", "waist", "waive", "waltz",
    "warty", "washy", "weary", "weave", "wedge", "weedy", "weigh", "weird", "welch", "welsh",
    "whack", "whale", "wharf", "wheat", "whelp", "whiff", "whine", "whiny", "whirl", "whisk",
    "whoop", "widen", "wider", "widow", "width", "wield", "wight", "willy", "wimpy", "wince",
    "winch", "windy", "wiser", "wispy", "witch", "witty", "woken", "woody", "wooer", "wooly",
    "woozy", "wordy", "wrack", "wrath", "wreak", "wreck", "wrest", "wring", "wrist", "wrung",
    "wryly", "yacht", "yearn", "yeast", "yodel", "yokel", "yummy", "zebra", "zesty", "zonal",
];

/// Returns true when `word` is an acceptable five-letter guess.
pub fn is_valid_guess(word: &str) -> bool {
    let lower = word.to_ascii_lowercase();
    ANSWERS.contains(&lower.as_str()) || EXTRA_VALID.contains(&lower.as_str())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn lists_are_non_empty() {
        assert!(!ANSWERS.is_empty());
        assert!(!EXTRA_VALID.is_empty());
    }

    #[test]
    fn every_word_is_five_lowercase_ascii_letters() {
        for &w in ANSWERS.iter().chain(EXTRA_VALID.iter()) {
            assert_eq!(w.len(), 5, "{w:?} is not five bytes long");
            assert!(
                w.bytes().all(|b| b.is_ascii_lowercase()),
                "{w:?} has non-lowercase-ascii characters"
            );
        }
    }

    #[test]
    fn no_duplicates_within_answers() {
        let set: HashSet<_> = ANSWERS.iter().collect();
        assert_eq!(set.len(), ANSWERS.len(), "ANSWERS contains duplicates");
    }

    #[test]
    fn no_duplicates_within_extra() {
        let set: HashSet<_> = EXTRA_VALID.iter().collect();
        assert_eq!(
            set.len(),
            EXTRA_VALID.len(),
            "EXTRA_VALID contains duplicates"
        );
    }

    #[test]
    fn answers_and_extra_do_not_overlap() {
        let answers: HashSet<_> = ANSWERS.iter().collect();
        for &w in EXTRA_VALID.iter() {
            assert!(!answers.contains(&w), "{w:?} appears in both lists");
        }
    }

    #[test]
    fn is_valid_guess_accepts_known_words() {
        assert!(is_valid_guess("crane"));
        assert!(is_valid_guess("sepia"));
        assert!(is_valid_guess("zonal"));
    }

    #[test]
    fn is_valid_guess_is_case_insensitive() {
        assert!(is_valid_guess("CRANE"));
        assert!(is_valid_guess("CrAnE"));
    }

    #[test]
    fn is_valid_guess_rejects_unknown_and_wrong_length() {
        assert!(!is_valid_guess("zzzzz"));
        assert!(!is_valid_guess("cat"));
        assert!(!is_valid_guess("craned"));
        assert!(!is_valid_guess(""));
    }
}
