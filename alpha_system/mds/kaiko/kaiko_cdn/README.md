## Deploy your Worker project

```bash
cd queng_system_mds/kaiko/kaiko_cdn

cargo install -q worker-build && worker-build --releas  
  
npx wrangler deploy
```

https://developers.cloudflare.com/workers/languages/rust/