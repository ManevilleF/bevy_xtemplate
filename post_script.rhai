let crate_name = variable::get("crate_name");

file::rename("apple/crate_name.xcodeproj", "apple/${crate_name}.xcodeproj");