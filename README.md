# Rust Fly.io Demo

Demo repo for my Rust Melbourne talk on deploying a Rust web server to Fly.io

---

Steps to deploy:

```bash
fly launch

# When smoke check fails
fly secrets set MESSAGE="Hello from Fly.io"

fly open
```
