print("Executing post script");

let crate_name = variable::get("crate_name");
let ios_enabled = variable::get("ios_enabled");

if ios_enabled {
	print(`Renaming apple/crate_name.xcodeproj to apple/${crate_name}.xcodeproj`);
	file::rename(`apple/crate_name.xcodeproj", "apple/${crate_name}.xcodeproj`);
}

print("Done");