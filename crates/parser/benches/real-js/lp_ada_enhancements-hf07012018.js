var engagementFix = (function() {
    var engEl;

    function focus(eng) {

        //console.log(eng.engagementType)

        // Run if a button was detected
        if (eng.engagementType === 5) {
            // Look in dom for a role of button.  We only need to do this once because each button load will cause this to run.
            setTimeout(function() {
                var button = document.querySelector('[role="button"]');
                button.removeAttribute('role');
            }, 200);
        }

        if (eng.engagementType === 23) {
            document.querySelectorAll('[data-LP-event="close"]')[0].click();
        }

        engEl = document.getElementById("chatEngagement"); 

        if (engEl != null) {
            //console.log('Focus: engEl found'); 

            setTimeout(function() {
                document.getElementById("chatEngagement").focus();
            }, 200);
        } else {
            //console.log('Focus: engEl missing');

            setTimeout(function() {
                document.getElementsByClassName('LPMcontainer')[0].removeAttribute('tabindex');
            }, 200);
        }
    };

    function retry() {
        //console.log('Retry: lpTag.events missing');
        
        setTimeout(function(){
            bind();
        }, 150);
    };

    function bind() {
        if (lpTag.events) {
            //console.log('bind: lpTag.events found');

            if (lpTag.events.bind) {
                //console.log('bind: lpTag.events.bind found');

                lpTag.events.bind('LP_OFFERS', 'OFFER_DISPLAY', focus);
            } else {
                retry();
            };
        } else {
            retry();
        };
    };

    return {
        start: bind
    };
}());

engagementFix.start();