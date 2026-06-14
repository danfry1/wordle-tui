# wordle-tui demo. Regenerate with:  demo/record.sh   (requires: vhs)
# Run via record.sh, not directly: it builds --release and exports
# $WORDLE_TUI_BIN. The solution is pinned to "crane" via WORDLE_TUI_SOLUTION so
# the scripted game (slate -> spare -> crane) always wins in three guesses.
Output wordle-tui.gif

Set Shell "bash"
Set FontSize 14
Set Width 640
Set Height 960
Set Padding 16
Set TypingSpeed 110ms
Set WindowBar Colorful

# --- off-camera: launch the game with a fixed answer ---
Hide
Type "WORDLE_TUI_SOLUTION=crane $WORDLE_TUI_BIN"
Enter
Sleep 1500ms
Show

# settle on the empty board
Sleep 1200ms

# guess 1: slate  (a, e land in place)
Type "slate"
Sleep 400ms
Enter
Sleep 1800ms

# guess 2: spare  (r shows up as present)
Type "spare"
Sleep 400ms
Enter
Sleep 1800ms

# guess 3: crane  — solved!
Type "crane"
Sleep 400ms
Enter
Sleep 2200ms

# copy the shareable result grid
Type "s"
Sleep 2500ms
