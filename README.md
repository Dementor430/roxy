# Roxy: Turbocharged Search Buddy 🔍  
*Find stuff fast. Combine results from everywhere.*  

![Roxy Demo](graphql.png)  
*(Works faster than your last Uber driver)*  

## Why Care? 🚨  
- **1 search = 8+ sites checked** (DuckDuckGo + Wikipedia + more soon)  
- **GraphQL magic** → Ask exactly what you want  
- **Rust-powered** → Sips memory like fine whiskey  
- **Your data stays yours** ← No creepy tracking  

```
# Ask Roxy for cat facts:
query {  
    search(request: {query: "why cats rule", engines: ["duckduckgo"]}) 
    {  
        results { title link }  
    }  
}
```

## Setup (3 Steps, 2 Beers Max) 🍻  
1. `git clone https://github.com/dementor430/roxy`  
2. `cd roxy`  
3. `cargo run --release`  
*Boom. Running at `http://localhost:8000`*  

**Nix Users:** Do `nix-shell` first if you're fancy  

## Tech Guts 🔩  
| Part           | Tools Used       | Why It Matters        |  
|----------------|------------------|-----------------------|  
| Web Framework  | Rocket           | Fast AF               |  
| GraphQL        | async-graphql    | No waiting around     |  
| HTML Parsing   | Scraper          | Reads websites like IKEA instructions |  

## Where Stuff Lives 🗂️  
```
roxy/  
├── 🚀 main.rs           # Starts the party  
├── 🔍 engines/          # Search detectives  
│   ├── duckduckgo.rs    # DuckDuckGo scraper  
│   ├── yacy.rs          # Yacy Api Integration  
│   └── wikimedia.rs     # Wikipedia API wizard  
└── 📦 models/           # Data containers
```

## Add New Search Engine (5min Hack) 👨💻  
```
// Want Reddit results? Add this:  
impl SearchEngine for RedditEngine {  
    async fn search(&self, query: &str) -> Result {  
        let memes = find_dank_memes(query); // 🐸  
        // ...return awesome results  
    }  
}
```

## Roadmap (Help Wanted) 🗺️  
- [ ] Add scoring system → Easier to prioritize  
- [ ] Cache results → Faster than light  
- [ ] Rate limits → Play nice with APIs  

## Contribute (Get Eternal Glory) 🏆  
1. Fork repo  
2. Add your magic  
3. Run `cargo test` (don't break stuff)  
4. Open PR → Profit!  

*Full rules: [CONTRIBUTING.md](link_here)*  

---  
⚡ **Powered by:** Rust nights + ☕ caffeine  
📜 **License:** MIT (Be cool, share changes)  
🐛 **Issues?** Tag with 🚨 if stuff is broken (Let's fix it together)
