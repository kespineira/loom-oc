cask "loom-oc" do
  version "0.1.0"
  sha256 "REPLACE_WITH_SHA256_FROM_GITHUB_RELEASE"

  url "https://github.com/kevin-espineira/loom-oc/releases/download/v#{version}/loom_#{version}_aarch64.dmg"
  name "Loom"
  desc "Visual config editor for OpenCode"
  homepage "https://github.com/kevin-espineira/loom-oc"

  app "Loom.app"

  zap trash: [
    "~/Library/Application Support/ai.loom.oc",
    "~/Library/Preferences/ai.loom.oc.plist",
    "~/Library/Caches/ai.loom.oc",
  ]
end
