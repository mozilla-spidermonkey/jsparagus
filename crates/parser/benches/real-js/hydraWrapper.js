    function generateHydraId(cg) {
    try {
      if (1) {
		  
            var d1 = new Date();
            var d1 = new Date();

                var options = {excludeDoNotTrack: true, excludePlugins: true};
                var fp = new Fingerprint2(options);
               fp.get(function(fpresult) {
                var d2 = new Date();
                var img = new Image(1,1);
                img.src = "https://svcs.ebay.com/delstats/email/DeviceFingerprint?cguid=" + cg + "&dfp=" + fpresult + "&hu=" + "hu" + "&timetaken=" + (d2 - d1) + "&rvr_id=" + 1234 + "&gcity=&gdma=1" +"&v=3";
            });
        }      
    } catch(err) {}
};