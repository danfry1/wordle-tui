# Reference Homebrew formula for wordle-tui (binary install, matches the
# waxon/marina formulae in danfry1/homebrew-tap).
#
# You normally DON'T edit this by hand: the release workflow
# (.github/workflows/release.yml) generates a fully-populated wordle-tui.rb
# with real sha256 checksums and attaches it to each GitHub release.
#
# To publish: download wordle-tui.rb from the release and commit it to the
# root of danfry1/homebrew-tap (alongside waxon.rb / marina.rb). Then:
#
#   brew install danfry1/tap/wordle-tui
#
# The <SHA256_*> placeholders below are filled in automatically by CI.
# typed: false
# frozen_string_literal: true

class WordleTui < Formula
  desc "Beautiful terminal word-guessing game built with ratatui"
  homepage "https://github.com/danfry1/wordle-tui"
  version "0.1.0"
  license "MIT"

  on_macos do
    if Hardware::CPU.intel?
      url "https://github.com/danfry1/wordle-tui/releases/download/v0.1.0/wordle-tui_0.1.0_darwin_amd64.tar.gz"
      sha256 "<SHA256_DARWIN_AMD64>"

      def install
        bin.install "wordle-tui"
      end
    end
    if Hardware::CPU.arm?
      url "https://github.com/danfry1/wordle-tui/releases/download/v0.1.0/wordle-tui_0.1.0_darwin_arm64.tar.gz"
      sha256 "<SHA256_DARWIN_ARM64>"

      def install
        bin.install "wordle-tui"
      end
    end
  end

  on_linux do
    if Hardware::CPU.intel? && Hardware::CPU.is_64_bit?
      url "https://github.com/danfry1/wordle-tui/releases/download/v0.1.0/wordle-tui_0.1.0_linux_amd64.tar.gz"
      sha256 "<SHA256_LINUX_AMD64>"

      def install
        bin.install "wordle-tui"
      end
    end
    if Hardware::CPU.arm? && Hardware::CPU.is_64_bit?
      url "https://github.com/danfry1/wordle-tui/releases/download/v0.1.0/wordle-tui_0.1.0_linux_arm64.tar.gz"
      sha256 "<SHA256_LINUX_ARM64>"

      def install
        bin.install "wordle-tui"
      end
    end
  end

  test do
    assert_match "wordle-tui #{version}", shell_output("#{bin}/wordle-tui --version")
  end
end
