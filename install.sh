if cargo -V 1>/dev/null 2>/dev/null; then
    if cargo build -r 1>/dev/null 2>/dev/null; then
        echo "\x1b[32;1msuccess\x1b[39m:\x1b[0m finished building binary"
        alias arcanum="$( cd -- "$(dirname "$0")" >/dev/null 2>&1 ; pwd -P )/target/release/arcanum"
        echo "\x1b[32;1msuccess\x1b[39m:\x1b[0m aliased binary as \`arcanum\`"
    else
        echo "\x1b[31;1merror\x1b[39m:\x1b[0m failed to build binary"
        echo "\x1b[35;1mfix\x1b[39m:\x1b[0m install the latest version of \`arcanum\` or file a bug report"
    fi
else
    echo "\x1b[31;1merror\x1b[39m:\x1b[0m failed to find \`cargo\`"
    echo "\x1b[35;1mfix\x1b[39m:\x1b[0m install \`cargo\` from \`https://rustup.rs/\`"
fi
