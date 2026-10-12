+++
title = "Changelog"
description = "Changelog"
weight = 2
+++

## [0.3.0](https://github.com/krivahtoo/stamon/compare/v0.2.0...v0.3.0) (2026-10-12)


### Features

* add docker and database checks ([#159](https://github.com/krivahtoo/stamon/issues/159)) ([f3ec318](https://github.com/krivahtoo/stamon/commit/f3ec318f54a9b2e1a3295e9e6d522cdd55d333eb))
* add tcp, dns, tls and push checks ([#156](https://github.com/krivahtoo/stamon/issues/156)) ([e07e714](https://github.com/krivahtoo/stamon/commit/e07e7142abe20e14a3ccb205058efb61ad2316b8))
* add uptime stats, tags, maintenance windows and status pages ([#158](https://github.com/krivahtoo/stamon/issues/158)) ([a817428](https://github.com/krivahtoo/stamon/commit/a817428d9d10b37a72a539ca375e7fb4cd755a58))
* Add ws auto reconnect ([7652af1](https://github.com/krivahtoo/stamon/commit/7652af12832d765656bdf88e590c5a8204fb6902))
* Add ws auto reconnect ([b6e56dd](https://github.com/krivahtoo/stamon/commit/b6e56dd0456bd4d511c0bdc775b9494c59c4be0f))
* **api:** Add dev auto reload support ([31fedf4](https://github.com/krivahtoo/stamon/commit/31fedf401619793f3fb2ff4f95ae42936b4b6d82))
* **api:** Add dev auto reload support ([3b24b8c](https://github.com/krivahtoo/stamon/commit/3b24b8ce057253f77f6d7a2dcde78c5e3622c0a6))
* **landing:** Update installation page ([cb049ff](https://github.com/krivahtoo/stamon/commit/cb049ff48ad0bf76989f66460bf164bdd2e45e94))
* **landing:** Update installation page ([6a4f4ec](https://github.com/krivahtoo/stamon/commit/6a4f4eca4b5303934ab9771640b7ec7a98e70b65))
* move checks into a crate with per-type JSON config ([#155](https://github.com/krivahtoo/stamon/issues/155)) ([f0a2229](https://github.com/krivahtoo/stamon/commit/f0a222935778b16d0a334eba12aa4e1bf63dc71a))
* send alerts to notification channels ([#157](https://github.com/krivahtoo/stamon/issues/157)) ([590a001](https://github.com/krivahtoo/stamon/commit/590a0017228a1043289160936ea81b08b5ff984c))
* **ui:** Add base search functionality ([9902e99](https://github.com/krivahtoo/stamon/commit/9902e9968204473fe7320b9c6a61106fcda892ad))
* **ui:** Update seach functionality ([4c4f2a1](https://github.com/krivahtoo/stamon/commit/4c4f2a1c5596ec75697437eafb91199e4cbd9035))


### Bug Fixes

* Add comprehensive unit tests across models, auth, utils, and SSL modules ([#143](https://github.com/krivahtoo/stamon/issues/143)) ([9e2f52c](https://github.com/krivahtoo/stamon/commit/9e2f52cf6f9c7ea45afda2603f47a2ffc73954f3))
* Add middlewares and extractors ([75fee09](https://github.com/krivahtoo/stamon/commit/75fee096ee4b5e9d6ddd20fa58b3ad4ddef457c0))
* address review findings from the merged stack ([#168](https://github.com/krivahtoo/stamon/issues/168)) ([d924e69](https://github.com/krivahtoo/stamon/commit/d924e69b438dccda2e28a148957a57d08e6f9ee3))
* allow dependency build scripts so Docker builds pass ([#169](https://github.com/krivahtoo/stamon/issues/169)) ([569e584](https://github.com/krivahtoo/stamon/commit/569e584391814d24ad9d8295cd02e31ea2d7cb6d))
* **auth:** avoid panic on invalid password hash ([#151](https://github.com/krivahtoo/stamon/issues/151)) ([650fea9](https://github.com/krivahtoo/stamon/commit/650fea9ff9537b955e5bb589dd51d079625d6062))
* Bump docker rust image ([#149](https://github.com/krivahtoo/stamon/issues/149)) ([8eba3db](https://github.com/krivahtoo/stamon/commit/8eba3db9fb7d9c15d28862ce95ac2601c3724044))
* **ci:** Update docker.yml ([15c1f6a](https://github.com/krivahtoo/stamon/commit/15c1f6ad6904c6cecd636ecd701d22e9f26d7af3))
* **ci:** Update docker.yml ([bc5f89d](https://github.com/krivahtoo/stamon/commit/bc5f89dd72d8adbba5666cf1ac66576452f8b737))
* **deps:** update axum monorepo ([77fc97d](https://github.com/krivahtoo/stamon/commit/77fc97d45f3fd95543ab3bb0a36d0761046b2c97))
* **deps:** update axum monorepo ([ae78b96](https://github.com/krivahtoo/stamon/commit/ae78b9630b7328c5270bb88bfcf356da65cc1369))
* **deps:** update dependency @unovis/svelte to v1.4.5 ([b279259](https://github.com/krivahtoo/stamon/commit/b27925930f3d7af16d68714fbc8aca68abef0767))
* **deps:** update dependency @unovis/svelte to v1.4.5 ([fa39863](https://github.com/krivahtoo/stamon/commit/fa398631273d43ed984b89b15f4a81772f1b4e0b))
* **deps:** update dependency @unovis/ts to v1.5.0 ([42ef237](https://github.com/krivahtoo/stamon/commit/42ef2370b9fb89d9553e96acabc08e873b0e930a))
* **deps:** update dependency @unovis/ts to v1.5.0 ([f109bb0](https://github.com/krivahtoo/stamon/commit/f109bb0399b4b6b4b89ddef757a3918e006c3d3b))
* **deps:** update dependency bits-ui to v0.21.16 ([de76bd8](https://github.com/krivahtoo/stamon/commit/de76bd83a2115a3394c3900470222fee0a29c6b1))
* **deps:** update dependency bits-ui to v0.21.16 ([2efab2e](https://github.com/krivahtoo/stamon/commit/2efab2e5c8e6c2353f361b3d0f4c407db97b1662))
* **deps:** update dependency lucide-svelte to ^0.468.0 ([7c6ac3c](https://github.com/krivahtoo/stamon/commit/7c6ac3c3dd64722bd6a707efc46a48e557fbecac))
* **deps:** update dependency lucide-svelte to ^0.468.0 ([0327248](https://github.com/krivahtoo/stamon/commit/032724824463c572cd205bcb110efc39b1814bbb))
* **deps:** update dependency svelte-sonner to v0.3.28 ([a544a6a](https://github.com/krivahtoo/stamon/commit/a544a6a0ab24927054ebabc310e9a90dd165d751))
* **deps:** update dependency svelte-sonner to v0.3.28 ([ade2290](https://github.com/krivahtoo/stamon/commit/ade229009b60c05cd16e7005a1e00e1f0220cb51))
* **deps:** update dependency tailwind-merge to v2.5.5 ([0a1fd7f](https://github.com/krivahtoo/stamon/commit/0a1fd7fd1c37b9092ad1b63bb4871d482cfb5a0a))
* **deps:** update dependency tailwind-merge to v2.5.5 ([a6a6048](https://github.com/krivahtoo/stamon/commit/a6a60483314c45f71eb7dd78f2ed7ee74c276eab))
* **deps:** update rust crate axum to v0.7.9 ([d65f1c1](https://github.com/krivahtoo/stamon/commit/d65f1c15305750e827ea7545a31ccb5fb8fb6084))
* **deps:** update rust crate axum to v0.7.9 ([b729348](https://github.com/krivahtoo/stamon/commit/b729348c9852d42c3348326c7b9337e11f3b31f0))
* **deps:** update rust crate axum-extra to v0.9.6 ([203e968](https://github.com/krivahtoo/stamon/commit/203e96867617bb9b6f23d51a1bf31401f5aaa2c2))
* **deps:** update rust crate axum-extra to v0.9.6 ([2731016](https://github.com/krivahtoo/stamon/commit/2731016513f871105aa6b9548d658555b09f737f))
* **deps:** update rust crate chrono to v0.4.39 ([ff14d83](https://github.com/krivahtoo/stamon/commit/ff14d83c3149767b1aa5c3d28eb9764d27bbb609))
* **deps:** update rust crate chrono to v0.4.39 ([a041bbf](https://github.com/krivahtoo/stamon/commit/a041bbf1300c21a1dcb4682b493c38e09e92371c))
* **deps:** update rust crate chrono to v0.4.40 ([6d4ade9](https://github.com/krivahtoo/stamon/commit/6d4ade959b6af6143a6a6c8b8c5bff8f7921ea56))
* **deps:** update rust crate chrono to v0.4.40 ([4a1a14b](https://github.com/krivahtoo/stamon/commit/4a1a14b1f37ff1ca9a392ee278a29e5aa2bbecb8))
* **deps:** update rust crate chrono-tz to v0.10.3 ([9a8288f](https://github.com/krivahtoo/stamon/commit/9a8288fbca987e9c5e237428974decf78e3ea8c2))
* **deps:** update rust crate chrono-tz to v0.10.3 ([d265ef7](https://github.com/krivahtoo/stamon/commit/d265ef74a129fc1e7a5930585135dfb190b49128))
* **deps:** update rust crate futures to v0.3.31 ([c3fa494](https://github.com/krivahtoo/stamon/commit/c3fa4945b96320579a2fe35718d2155ce60f030b))
* **deps:** update rust crate futures to v0.3.31 ([f7dc556](https://github.com/krivahtoo/stamon/commit/f7dc55637c62cef9d9a4a7cc922aca158fc3123d))
* **deps:** update rust crate jsonwebtoken to v9.3.1 ([9d239d3](https://github.com/krivahtoo/stamon/commit/9d239d3fbe8530eee15099d7892ae04f4bdf7999))
* **deps:** update rust crate jsonwebtoken to v9.3.1 ([51f60f7](https://github.com/krivahtoo/stamon/commit/51f60f78632670c895dd93a364fbd1728369d50b))
* **deps:** update rust crate openssl to v0.10.68 ([cbecc25](https://github.com/krivahtoo/stamon/commit/cbecc256dd473d5d26ba00679fa517415d2bf37f))
* **deps:** update rust crate openssl to v0.10.68 ([e13189e](https://github.com/krivahtoo/stamon/commit/e13189e89fd1978f082ef9df4fef1d63d748f128))
* **deps:** update rust crate reqwest to v0.12.12 ([b8b6df0](https://github.com/krivahtoo/stamon/commit/b8b6df0cd906a29e61119cd8dc51ca6b6695d79d))
* **deps:** update rust crate reqwest to v0.12.12 ([fb61263](https://github.com/krivahtoo/stamon/commit/fb61263b3aff23bb8e4159976ed5c74398afa9a5))
* **deps:** update rust crate serde to v1.0.217 ([721a448](https://github.com/krivahtoo/stamon/commit/721a4480385ecef75a694e3549da05303a16f9a0))
* **deps:** update rust crate serde to v1.0.217 ([8e91416](https://github.com/krivahtoo/stamon/commit/8e914163ff5a2ae4f8b31c719bc059100da54aa2))
* **deps:** update rust crate serde_json to v1.0.135 ([1cb4c48](https://github.com/krivahtoo/stamon/commit/1cb4c4878a0c8b603c989e569c5f14c3c86d9049))
* **deps:** update rust crate serde_json to v1.0.135 ([ec4e412](https://github.com/krivahtoo/stamon/commit/ec4e412e7e814516a2eaa2b4d0a89c40344615dd))
* **deps:** update rust crate thiserror to v1.0.69 ([ad13ee3](https://github.com/krivahtoo/stamon/commit/ad13ee3e984ba2809d7fbaf2a257fd05650b43f3))
* **deps:** update rust crate thiserror to v1.0.69 ([6dafe47](https://github.com/krivahtoo/stamon/commit/6dafe475d9331f7e9a1cfc606611bbbe3578d6a3))
* **deps:** update tokio-tracing monorepo ([c1aa1fc](https://github.com/krivahtoo/stamon/commit/c1aa1fca3ec62646fb979b574afe46d4f9b89cca))
* **deps:** update tokio-tracing monorepo ([25cb7c7](https://github.com/krivahtoo/stamon/commit/25cb7c795ef0274adcec539696a5e2c2e88e9399))
* docker build ([657f9f0](https://github.com/krivahtoo/stamon/commit/657f9f00b678c4c34bb667e0ebe19368cbbacb44))
* docker build ([69e616d](https://github.com/krivahtoo/stamon/commit/69e616d57922f88429f19e4157f0afe94c598787))
* Fix fall through issue ([3770eae](https://github.com/krivahtoo/stamon/commit/3770eae004f1df4dbff17d95b3057d79e9f7af23))
* hide push tokens from viewers ([#161](https://github.com/krivahtoo/stamon/issues/161)) ([7fb06a0](https://github.com/krivahtoo/stamon/commit/7fb06a04ed1453ae7d1b6fa83a4129ab20305937))
* make monitoring checks run and enforce user roles ([#154](https://github.com/krivahtoo/stamon/issues/154)) ([8f2dea3](https://github.com/krivahtoo/stamon/commit/8f2dea310e745056d03dc7e0bfa9ea1d99dd5253))
* refactor and update ui ([d5b02cd](https://github.com/krivahtoo/stamon/commit/d5b02cd5a1c380849a580a7ead59636497a3be6a))
* refactor and update ui ([7788013](https://github.com/krivahtoo/stamon/commit/778801304bf49fc1a8fb2986a06e1fb53dcdb621))
* Update database schema and dependencies ([bc9a185](https://github.com/krivahtoo/stamon/commit/bc9a185afa519e4613bc70a4a090a5cceb69c0f4))
* Update database schema and dependencies ([ea1944b](https://github.com/krivahtoo/stamon/commit/ea1944b8bc1239310f61a7f3becf15d7b2e0d954))
* Update error handling ([f323da3](https://github.com/krivahtoo/stamon/commit/f323da31867f4a3c446ce5146b06c0066ee5b3e5))
* Update error handling ([5092496](https://github.com/krivahtoo/stamon/commit/509249691900075c762ad8f1c7acc96dacb8b424))
* Update landing ([9ac9abe](https://github.com/krivahtoo/stamon/commit/9ac9abe697fea658bc3dba4804b4612115d6bb85))
* Update landing ([90db1b2](https://github.com/krivahtoo/stamon/commit/90db1b21d158c8dba2f27a85b4d98f0fd49711fe))
* use per-architecture cargo caches in the Docker build ([#170](https://github.com/krivahtoo/stamon/issues/170)) ([4ee3f83](https://github.com/krivahtoo/stamon/commit/4ee3f8350abb54ac831c9ccc57b9a1f1cfbbad10))

## v0.1.0 - 2024-09-06

- First release!
