#!/bin/sh

contains()
{
    local str="$1" substr="$2"
    [ "$str" = "$substr" -o -z "${str##$substr:*}" -o -z "${str##*:$substr:*}" -o -z "${str%%*:$substr}" ]
}

if [ -z "$XDG_DATA_HOME" ]; then
    export XDG_DATA_HOME="$HOME/.local/share"
fi

if [ -z "$XDG_CONFIG_HOME" ]; then
    export XDG_CONFIG_HOME="$HOME/.config"
fi

if [ -z "$XDG_DATA_DIRS" ]; then
    XDG_DATA_DIRS="$XDG_DATA_HOME:/usr/local/share:/usr/share"
else
    if ! contains "$XDG_DATA_DIRS" "/usr/share"; then
        XDG_DATA_DIRS="$XDG_DATA_DIRS:/usr/share"
    fi
fi
export XDG_DATA_DIRS

if [ -z "$XDG_CONFIG_DIRS" ]; then
    export XDG_CONFIG_DIRS="/etc:/etc/xdg:/usr/share"
else
    if ! contains "$XDG_CONFIG_DIRS" '/etc/xdg'; then
        XDG_CONFIG_DIRS="$XDG_CONFIG_DIRS:/etc/xdg"
    fi
fi

if [ -z "$XDG_CACHE_HOME" ]; then
    export XDG_CACHE_HOME="$HOME/.cache"
fi

# Ensure the existence of the 'Desktop' folder
if [ -e "$XDG_CONFIG_HOME/user-dirs.dirs" ]; then
    . "$XDG_CONFIG_HOME/user-dirs.dirs"
else
    XDG_DESKTOP_DIR="$HOME/Desktop"
fi
mkdir -p "$XDG_DESKTOP_DIR"

# Clean up after GDM (GDM sets the number of desktops to one)
xprop -root -remove _NET_NUMBER_OF_DESKTOPS -remove _NET_DESKTOP_NAMES -remove _NET_CURRENT_DESKTOP 2> /dev/null

# Launch DBus if needed
if [ -z "$DBUS_SESSION_BUS_ADDRESS" ]; then
    if [ -z "$XDG_RUNTIME_DIR" ] || ! [ -S "$XDG_RUNTIME_DIR/bus" ] || ! [ -O "$XDG_RUNTIME_DIR/bus" ]; then
        eval "$(dbus-launch --sh-syntax --exit-with-session)" || echo "startkmp: error executing dbus-launch" >&2
    fi
fi

# Qt4 platform plugin
export QT_PLATFORM_PLUGIN=LDE

# Qt5 platform plugin
export QT_QPA_PLATFORMTHEME=LDE

export XDG_CURRENT_DESKTOP="LDE"

# Start the LDE session
exec lde_session
