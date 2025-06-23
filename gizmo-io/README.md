### Latest Status:
`cargo test -p gizmo-io --features happy_paths`

These are macro-heavy tests that severely lag my aging rig when changing certain pieces of heavily-referenced code, so,
we've feature-gated them.

Right now all I have are accuracy tests, ensuring we're correctly handling different contexts while serde-ing. You'll
find them in `formats/graphson/tests/sanity/`. They're to ensure we're handling different contexts correctly and only 
check equality between the graphson we feed it and the test's corresponding GValue object. _Eventually_ we'll have some
more thorough tests.