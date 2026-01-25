#!/bin/sh

set -e

PROJECT_NAME="uwais"
VERSION=$(grep '^version' Cargo.toml | head -n1 | cut -d '"' -f2)
DIST_DIR="dist"

TARGETS="
x86_64-unknown-linux-gnu
x86_64-pc-windows-gnu
"

echo "📦 Building $PROJECT_NAME v$VERSION"
echo "🧹 Cleaning dist..."
rm -rf "$DIST_DIR"
mkdir -p "$DIST_DIR"

for TARGET in $TARGETS; do
    echo "🚧 Building for $TARGET..."
    cross build --release --target "$TARGET"

    BIN_NAME="$PROJECT_NAME"
    [ "$TARGET" = "x86_64-pc-windows-gnu" ] && BIN_NAME="${BIN_NAME}.exe"

    BIN_PATH="target/$TARGET/release/$BIN_NAME"

    if [ ! -f "$BIN_PATH" ]; then
        echo "❌ Binary not found: $BIN_PATH"
        continue
    fi

    case "$TARGET" in
        x86_64-unknown-linux-gnu)   SUFFIX="x86_64-linux" ;;
        x86_64-pc-windows-gnu)      SUFFIX="x86_64-windows" ;;
        *)                          SUFFIX="$TARGET" ;;
    esac

    PKG_NAME="${PROJECT_NAME}-v${VERSION}-${SUFFIX}"
    PKG_DIR="${DIST_DIR}/${PKG_NAME}"
    mkdir -p "$PKG_DIR"

    cp "$BIN_PATH" "$PKG_DIR/"

    if echo "$TARGET" | grep -q "windows"; then
        zip -r "$DIST_DIR/${PKG_NAME}.zip" -j "$PKG_DIR"
    else
        tar -czf "$DIST_DIR/${PKG_NAME}.tar.gz" -C "$PKG_DIR" .
        zip -r "$DIST_DIR/${PKG_NAME}.zip" -j "$PKG_DIR"
    fi

    rm -rf "$PKG_DIR"

    if echo "$BIN_NAME" | grep -q ".exe"; then
      cp "$BIN_PATH" "$DIST_DIR/${PROJECT_NAME}-v${VERSION}-${SUFFIX}.exe"
    fi

    echo "✅ Packaged: $DIST_DIR/${PKG_NAME}.[zip|tar.gz]"
done

echo "🎉 All done. See $DIST_DIR/"
