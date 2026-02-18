# 🛠️ Auditoría de Software 3026

| Métrica | Valor |
| :--- | :--- |
| **Líneas de Código (Netas)** | 10174 LoC |
| **Peso Total del Proyecto** | 788.54KB |
| **Estado de Sintonía** | Activa |

### Mapa de Arquitectura y Pesos
```text
├── .env.example (0 LoC | 101.00B)
├── .github/ [549.00B]
│   └── workflows/ [549.00B]
│       ├── .gitkeep (0 LoC | 0.00B)
│       └── ci.yml (22 LoC | 549.00B)
├── Justfile (0 LoC | 1008.00B)
├── README.md (0 LoC | 1.87KB)
├── backend/ [263.96KB]
│   ├── .sqlx/ [0.00B]
│   ├── Cargo.lock (0 LoC | 103.49KB)
│   ├── Cargo.toml (43 LoC | 1.49KB)
│   ├── Dockerfile (0 LoC | 998.00B)
│   ├── config/ [87.00B]
│   │   └── default.toml (4 LoC | 87.00B)
│   ├── migrations/ [4.85KB]
│   │   ├── 0001_init.sql (5 LoC | 168.00B)
│   │   ├── 0002_add_password_hash.sql (2 LoC | 107.00B)
│   │   ├── 0003_add_role_to_users.sql (4 LoC | 194.00B)
│   │   ├── 0004_create_audit_logs.sql (8 LoC | 398.00B)
│   │   ├── 0005_add_email.sql (3 LoC | 193.00B)
│   │   ├── 0006_add_avatar_url.sql (3 LoC | 113.00B)
│   │   ├── 0007_create_refresh_tokens.sql (13 LoC | 524.00B)
│   │   ├── 0008_create_password_reset_tokens.sql (13 LoC | 566.00B)
│   │   ├── 0009_add_email_verification.sql (16 LoC | 722.00B)
│   │   └── 0010_create_rbac_tables.sql (38 LoC | 1.94KB)
│   ├── src/ [142.51KB]
│   │   ├── api/ [63.32KB]
│   │   │   ├── handlers/ [61.26KB]
│   │   │   │   ├── .gitkeep (0 LoC | 0.00B)
│   │   │   │   ├── audit.rs (64 LoC | 2.12KB)
│   │   │   │   ├── auth.rs (225 LoC | 7.88KB)
│   │   │   │   ├── common.rs (52 LoC | 1.71KB)
│   │   │   │   ├── dashboard.rs (46 LoC | 1.34KB)
│   │   │   │   ├── mod.rs (20 LoC | 648.00B)
│   │   │   │   ├── roles.rs (109 LoC | 3.64KB)
│   │   │   │   ├── user.rs (939 LoC | 37.23KB)
│   │   │   │   └── users.rs (192 LoC | 6.72KB)
│   │   │   ├── middleware.rs (60 LoC | 2.02KB)
│   │   │   ├── mod.rs (2 LoC | 38.00B)
│   │   │   └── routes/ [0.00B]
│   │   │       └── .gitkeep (0 LoC | 0.00B)
│   │   ├── core/ [35.27KB]
│   │   │   ├── container.rs (93 LoC | 3.12KB)
│   │   │   ├── mod.rs (4 LoC | 73.00B)
│   │   │   ├── models/ [5.96KB]
│   │   │   │   ├── .gitkeep (0 LoC | 0.00B)
│   │   │   │   ├── mod.rs (1 LoC | 15.00B)
│   │   │   │   └── user.rs (205 LoC | 5.94KB)
│   │   │   ├── repository.rs (67 LoC | 3.98KB)
│   │   │   └── services/ [22.14KB]
│   │   │       ├── .gitkeep (0 LoC | 0.00B)
│   │   │       ├── audit_service.rs (91 LoC | 3.38KB)
│   │   │       ├── auth_service.rs (275 LoC | 10.67KB)
│   │   │       ├── mod.rs (4 LoC | 89.00B)
│   │   │       ├── role_service.rs (55 LoC | 1.98KB)
│   │   │       └── user_service.rs (166 LoC | 6.02KB)
│   │   ├── data/ [25.00KB]
│   │   │   ├── audit_repository.rs (34 LoC | 1.10KB)
│   │   │   ├── mod.rs (162 LoC | 7.00KB)
│   │   │   ├── rbac_repository.rs (103 LoC | 4.31KB)
│   │   │   ├── repositories/ [0.00B]
│   │   │   │   └── .gitkeep (0 LoC | 0.00B)
│   │   │   ├── token_repository.rs (113 LoC | 4.38KB)
│   │   │   └── user_repository.rs (198 LoC | 8.22KB)
│   │   ├── error.rs (44 LoC | 1.37KB)
│   │   ├── lib.rs (170 LoC | 7.03KB)
│   │   ├── main.rs (68 LoC | 2.62KB)
│   │   ├── services/ [7.01KB]
│   │   │   ├── email.rs (157 LoC | 7.00KB)
│   │   │   └── mod.rs (1 LoC | 14.00B)
│   │   ├── settings.rs (24 LoC | 910.00B)
│   │   └── shared/ [0.00B]
│   │       └── .gitkeep (0 LoC | 0.00B)
│   └── tests/ [10.57KB]
│       ├── .gitkeep (0 LoC | 0.00B)
│       └── integration_tests.rs (331 LoC | 10.57KB)
├── docs/ [899.00B]
│   ├── ALCANCE_MVP.md (0 LoC | 899.00B)
│   ├── adr/ [0.00B]
│   │   └── .gitkeep (0 LoC | 0.00B)
│   ├── api/ [0.00B]
│   │   └── .gitkeep (0 LoC | 0.00B)
│   └── database/ [0.00B]
│       └── .gitkeep (0 LoC | 0.00B)
├── frontend/ [507.01KB]
│   ├── .github/ [669.00B]
│   │   └── workflows/ [669.00B]
│   │       └── playwright.yml (27 LoC | 669.00B)
│   ├── Dockerfile (0 LoC | 522.00B)
│   ├── README.md (0 LoC | 1.62KB)
│   ├── astro.config.mjs (0 LoC | 185.00B)
│   ├── package-lock.json (0 LoC | 269.44KB)
│   ├── package.json (0 LoC | 989.00B)
│   ├── playwright.config.ts (70 LoC | 2.13KB)
│   ├── postcss.config.mjs (0 LoC | 69.00B)
│   ├── public/ [1.37KB]
│   │   ├── favicon.ico (0 LoC | 655.00B)
│   │   └── favicon.svg (0 LoC | 749.00B)
│   ├── src/ [227.55KB]
│   │   ├── assets/ [4.27KB]
│   │   │   ├── astro.svg (0 LoC | 2.85KB)
│   │   │   └── background.svg (0 LoC | 1.42KB)
│   │   ├── components/ [124.79KB]
│   │   │   ├── AuditTable.astro (57 LoC | 2.97KB)
│   │   │   ├── LoginForm.astro (57 LoC | 2.95KB)
│   │   │   ├── LogoutButton.astro (33 LoC | 1.50KB)
│   │   │   ├── UserForm.astro (62 LoC | 2.59KB)
│   │   │   ├── UserList.astro (98 LoC | 5.07KB)
│   │   │   ├── Welcome.astro (183 LoC | 4.81KB)
│   │   │   ├── audit/ [11.78KB]
│   │   │   │   ├── AuditFilters.astro (38 LoC | 1.45KB)
│   │   │   │   ├── AuditTimeline.astro (27 LoC | 809.00B)
│   │   │   │   └── audit.ts (168 LoC | 9.54KB)
│   │   │   ├── dashboard/ [13.32KB]
│   │   │   │   ├── roles/ [11.15KB]
│   │   │   │   │   └── roles-matrix.tsx (238 LoC | 11.15KB)
│   │   │   │   └── users/ [2.17KB]
│   │   │   │       └── delete-user-dialog.tsx (57 LoC | 2.17KB)
│   │   │   ├── layout/ [17.57KB]
│   │   │   │   ├── dashboard-layout.astro (62 LoC | 2.13KB)
│   │   │   │   ├── header.astro (54 LoC | 2.75KB)
│   │   │   │   └── sidebar.astro (237 LoC | 12.68KB)
│   │   │   ├── settings/ [21.30KB]
│   │   │   │   ├── AppearanceTab.astro (59 LoC | 3.77KB)
│   │   │   │   ├── ProfileTab.astro (116 LoC | 4.15KB)
│   │   │   │   ├── SecurityTab.astro (76 LoC | 2.97KB)
│   │   │   │   ├── events.ts (108 LoC | 4.63KB)
│   │   │   │   ├── settings.ts (73 LoC | 3.43KB)
│   │   │   │   ├── tabs.ts (17 LoC | 816.00B)
│   │   │   │   └── theme.ts (40 LoC | 1.54KB)
│   │   │   └── ui/ [40.95KB]
│   │   │       ├── Toast.astro (63 LoC | 3.47KB)
│   │   │       ├── avatar.astro (40 LoC | 751.00B)
│   │   │       ├── badge.astro (25 LoC | 868.00B)
│   │   │       ├── button.astro (64 LoC | 1.70KB)
│   │   │       ├── button.tsx (51 LoC | 1.85KB)
│   │   │       ├── card-content.astro (10 LoC | 189.00B)
│   │   │       ├── card-description.astro (10 LoC | 206.00B)
│   │   │       ├── card-footer.astro (10 LoC | 207.00B)
│   │   │       ├── card-header.astro (10 LoC | 210.00B)
│   │   │       ├── card-title.astro (10 LoC | 229.00B)
│   │   │       ├── card.astro (10 LoC | 237.00B)
│   │   │       ├── card.tsx (71 LoC | 1.91KB)
│   │   │       ├── command-menu.tsx (162 LoC | 6.00KB)
│   │   │       ├── command.tsx (136 LoC | 4.87KB)
│   │   │       ├── dialog.tsx (108 LoC | 3.86KB)
│   │   │       ├── empty-state.tsx (41 LoC | 1.12KB)
│   │   │       ├── index.ts (11 LoC | 591.00B)
│   │   │       ├── input.astro (52 LoC | 1.23KB)
│   │   │       ├── skeleton.tsx (13 LoC | 278.00B)
│   │   │       └── table.astro (282 LoC | 11.25KB)
│   │   ├── config.ts (4 LoC | 231.00B)
│   │   ├── env.d.ts (11 LoC | 206.00B)
│   │   ├── layouts/ [1.04KB]
│   │   │   └── Layout.astro (31 LoC | 1.04KB)
│   │   ├── lib/ [13.34KB]
│   │   │   ├── api.ts (382 LoC | 12.21KB)
│   │   │   └── utils.ts (37 LoC | 1.13KB)
│   │   ├── middleware.ts (47 LoC | 1.67KB)
│   │   ├── pages/ [73.45KB]
│   │   │   ├── 404.astro (37 LoC | 1.69KB)
│   │   │   ├── 500.astro (37 LoC | 1.70KB)
│   │   │   ├── dashboard/ [37.66KB]
│   │   │   │   ├── audit.astro (186 LoC | 8.29KB)
│   │   │   │   ├── roles.astro (39 LoC | 1.27KB)
│   │   │   │   ├── settings.astro (112 LoC | 5.55KB)
│   │   │   │   ├── users/ [10.02KB]
│   │   │   │   │   ├── [id]/ [3.92KB]
│   │   │   │   │   │   └── details.astro (78 LoC | 3.92KB)
│   │   │   │   │   └── [id].astro (124 LoC | 6.10KB)
│   │   │   │   └── users.astro (216 LoC | 12.52KB)
│   │   │   ├── dashboard.astro (115 LoC | 4.29KB)
│   │   │   ├── forgot-password.astro (119 LoC | 5.16KB)
│   │   │   ├── index.astro (21 LoC | 955.00B)
│   │   │   ├── login.astro (117 LoC | 4.58KB)
│   │   │   ├── logout.astro (28 LoC | 995.00B)
│   │   │   ├── register.astro (135 LoC | 5.11KB)
│   │   │   ├── reset-password.astro (161 LoC | 7.43KB)
│   │   │   └── verify-email.astro (74 LoC | 3.94KB)
│   │   ├── stores/ [4.79KB]
│   │   │   ├── auth.ts (3 LoC | 158.00B)
│   │   │   ├── table.ts (161 LoC | 3.97KB)
│   │   │   └── toast.ts (23 LoC | 678.00B)
│   │   ├── styles/ [2.45KB]
│   │   │   └── globals.css (77 LoC | 2.45KB)
│   │   └── types/ [1.33KB]
│   │       └── index.ts (70 LoC | 1.33KB)
│   ├── tests/ [2.31KB]
│   │   ├── auth.spec.ts (34 LoC | 1.75KB)
│   │   └── example.spec.ts (13 LoC | 583.00B)
│   └── tsconfig.json (0 LoC | 211.00B)
└── infra/ [13.20KB]
    ├── docker/ [0.00B]
    │   └── .gitkeep (0 LoC | 0.00B)
    ├── prod/ [2.55KB]
    │   ├── Caddyfile (0 LoC | 463.00B)
    │   ├── docker-compose.yml (42 LoC | 1.02KB)
    │   ├── recovery.py (0 LoC | 0.00B)
    │   ├── setup_server.sh (0 LoC | 1.08KB)
    │   └── vigilante.sh (0 LoC | 0.00B)
    └── scripts/ [10.65KB]
        ├── .gitkeep (0 LoC | 0.00B)
        ├── admin_promote.py (19 LoC | 702.00B)
        ├── consultor.py (81 LoC | 3.69KB)
        ├── debug_audit.py (16 LoC | 510.00B)
        ├── debug_users.py (12 LoC | 387.00B)
        ├── ghost_hunter.py (0 LoC | 0.00B)
        ├── ignition.py (0 LoC | 0.00B)
        ├── seed_users.py (31 LoC | 1.59KB)
        ├── semilla.py (75 LoC | 2.88KB)
        ├── shield.py (0 LoC | 0.00B)
        └── ver_logs.py (21 LoC | 941.00B)
```
