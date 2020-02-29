_satellite.pushAsyncScript(function(event, target, $variables){
  (function() {
  var ccLoop = setInterval(function() {
    if (window.cookieconsent) {
      cookieconsent("consent", "Publicité ciblée", function(accept) {
        switch (accept) {
          case "accept":
            var pages = ["lci.fr", "valwebfrontk8s.etf1.tf1.fr", "localhost"];
            var page_taggee = false;
            var i;
            for (i = 0; i < pages.length; i++) {
              if (document.URL.indexOf(pages[i]) != -1) {
                page_taggee = true;
              }
            }

            if (page_taggee == true) {
              var tag = document.createElement('script');
              tag.src = 'https://static.freeskreen.com/publisher/3099/freeskreen.min.js';
              document.body.appendChild(tag);
            }

            clearInterval(ccLoop);
            break;
          case "refuse":
            clearInterval(ccLoop);
            break;
        }
      });
    };
  }, 1000);
})();
});
