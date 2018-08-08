(function() {var implementors = {};
implementors["erased_serde"] = [{text:"impl&lt;'erased&gt; <a class=\"trait\" href=\"serde/ser/trait.Serialize.html\" title=\"trait serde::ser::Serialize\">Serialize</a> for <a class=\"trait\" href=\"erased_serde/trait.Serialize.html\" title=\"trait erased_serde::Serialize\">Serialize</a> + 'erased",synthetic:false,types:["erased_serde::ser::Serialize"]},{text:"impl&lt;'erased&gt; <a class=\"trait\" href=\"serde/ser/trait.Serialize.html\" title=\"trait serde::ser::Serialize\">Serialize</a> for <a class=\"trait\" href=\"erased_serde/trait.Serialize.html\" title=\"trait erased_serde::Serialize\">Serialize</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> + 'erased",synthetic:false,types:["erased_serde::ser::Serialize"]},{text:"impl&lt;'erased&gt; <a class=\"trait\" href=\"serde/ser/trait.Serialize.html\" title=\"trait serde::ser::Serialize\">Serialize</a> for <a class=\"trait\" href=\"erased_serde/trait.Serialize.html\" title=\"trait erased_serde::Serialize\">Serialize</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html\" title=\"trait core::marker::Sync\">Sync</a> + 'erased",synthetic:false,types:["erased_serde::ser::Serialize"]},{text:"impl&lt;'erased&gt; <a class=\"trait\" href=\"serde/ser/trait.Serialize.html\" title=\"trait serde::ser::Serialize\">Serialize</a> for <a class=\"trait\" href=\"erased_serde/trait.Serialize.html\" title=\"trait erased_serde::Serialize\">Serialize</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html\" title=\"trait core::marker::Sync\">Sync</a> + 'erased",synthetic:false,types:["erased_serde::ser::Serialize"]},];
implementors["serde_json"] = [{text:"impl <a class=\"trait\" href=\"serde/ser/trait.Serialize.html\" title=\"trait serde::ser::Serialize\">Serialize</a> for <a class=\"struct\" href=\"serde_json/map/struct.Map.html\" title=\"struct serde_json::map::Map\">Map</a>&lt;<a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/alloc/string/struct.String.html\" title=\"struct alloc::string::String\">String</a>, <a class=\"enum\" href=\"serde_json/value/enum.Value.html\" title=\"enum serde_json::value::Value\">Value</a>&gt;",synthetic:false,types:["serde_json::map::Map"]},{text:"impl <a class=\"trait\" href=\"serde/ser/trait.Serialize.html\" title=\"trait serde::ser::Serialize\">Serialize</a> for <a class=\"enum\" href=\"serde_json/value/enum.Value.html\" title=\"enum serde_json::value::Value\">Value</a>",synthetic:false,types:["serde_json::value::Value"]},{text:"impl <a class=\"trait\" href=\"serde/ser/trait.Serialize.html\" title=\"trait serde::ser::Serialize\">Serialize</a> for <a class=\"struct\" href=\"serde_json/struct.Number.html\" title=\"struct serde_json::Number\">Number</a>",synthetic:false,types:["serde_json::number::Number"]},];
implementors["serde_tagged"] = [{text:"impl&lt;'b, T:&nbsp;?<a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html\" title=\"trait core::marker::Sized\">Sized</a>&gt; <a class=\"trait\" href=\"serde/ser/trait.Serialize.html\" title=\"trait serde::ser::Serialize\">Serialize</a> for <a class=\"struct\" href=\"serde_tagged/util/erased/struct.SerializeErased.html\" title=\"struct serde_tagged::util::erased::SerializeErased\">SerializeErased</a>&lt;'b, T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: <a class=\"trait\" href=\"erased_serde/ser/trait.Serialize.html\" title=\"trait erased_serde::ser::Serialize\">Serialize</a>,&nbsp;</span>",synthetic:false,types:["serde_tagged::util::erased::SerializeErased"]},{text:"impl&lt;'a&gt; <a class=\"trait\" href=\"serde/ser/trait.Serialize.html\" title=\"trait serde::ser::Serialize\">Serialize</a> for <a class=\"enum\" href=\"serde_tagged/util/enum.TagString.html\" title=\"enum serde_tagged::util::TagString\">TagString</a>&lt;'a&gt;",synthetic:false,types:["serde_tagged::util::TagString"]},];
implementors["slog_conf"] = [{text:"impl <a class=\"trait\" href=\"serde/ser/trait.Serialize.html\" title=\"trait serde::ser::Serialize\">Serialize</a> for <a class=\"enum\" href=\"slog_conf/common/enum.Target.html\" title=\"enum slog_conf::common::Target\">Target</a>",synthetic:false,types:["slog_conf::common::Target"]},{text:"impl <a class=\"trait\" href=\"serde/ser/trait.Serialize.html\" title=\"trait serde::ser::Serialize\">Serialize</a> for <a class=\"enum\" href=\"slog_conf/common/enum.Level.html\" title=\"enum slog_conf::common::Level\">Level</a>",synthetic:false,types:["slog_conf::common::Level"]},{text:"impl&lt;'a&gt; <a class=\"trait\" href=\"serde/ser/trait.Serialize.html\" title=\"trait serde::ser::Serialize\">Serialize</a> for <a class=\"trait\" href=\"slog_conf/trait.Config.html\" title=\"trait slog_conf::Config\">Config</a> + 'a",synthetic:false,types:["slog_conf::Config"]},];

            if (window.register_implementors) {
                window.register_implementors(implementors);
            } else {
                window.pending_implementors = implementors;
            }
        
})()