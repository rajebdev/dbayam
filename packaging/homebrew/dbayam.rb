# Homebrew cask for DBayam.
# Publish the notarized DMG produced by scripts/bundle-macos.sh to the release page,
# then update `version` and `sha256` (printed by the script) and submit to a tap:
#   brew tap dbayam/tap && brew install --cask dbayam
cask "dbayam" do
  version "0.3.48"
  sha256 "d1f3c2a2881bfc5430f6814c05252d9958f39c7b27cbccff16e12d69b859331e"

  url "https://github.com/rajebdev/dbayam/releases/download/v#{version}/DBayam-#{version}.dmg"
  name "DBayam"
  desc "Lightweight SQL database client written in Rust"
  homepage "https://github.com/rajebdev/dbayam"

  depends_on macos: ">= :monterey"

  app "DBayam.app"
  binary "#{appdir}/DBayam.app/Contents/MacOS/dbayam"

  zap trash: [
    "~/Library/Application Support/dev.dbayam.DBayam",
    "~/Library/Caches/dev.dbayam.DBayam",
    "~/Library/Preferences/dev.dbayam.DBayam.plist",
  ]
end
