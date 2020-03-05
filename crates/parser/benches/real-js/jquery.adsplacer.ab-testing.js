jQuery(document).ready(function(){
    setTimeout(function(){
        adsplacerTrackIframeClick();
        adsplacerTrackAdClick();
    }, 2000);
    var ads = jQuery('.adsplaser_pro_abtest');
    for(var i = 0; i < ads.length; i++){
        adsplacerViewAd(ads[i]);
    }
    jQuery(window).scroll(function(){
        if (window.adsplacerScrollTimeout) clearTimeout(window.adsplacerScrollTimeout);
        window.adsplacerScrollTimeout = setTimeout(function(){
            var ads = jQuery('.adsplaser_pro_abtest');
            for(var i = 0; i < ads.length; i++){
                adsplacerViewAd(ads[i]);
            }
        }, 500);
    });
});