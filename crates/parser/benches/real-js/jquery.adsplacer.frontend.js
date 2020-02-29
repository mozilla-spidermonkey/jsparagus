function AdsplacerProReadCookie(name) {
    var nameEQ = name + "=";
    var ca = document.cookie.split(';');
    for (var i = 0; i < ca.length; i++) {
        var c = ca[i];
        while (c.charAt(0) == ' ') c = c.substring(1, c.length);
        if (c.indexOf(nameEQ) == 0) return c.substring(nameEQ.length, c.length);
    }
    return null;
}

function AdsplacerProSetCookie(name, value, time) {
    if(typeof time === 'undefined'){
        time = 30;
    }
    var date = new Date(),
        expires = 'expires=';
    date.setTime(date.setDate(date.getDate() + time));
    expires += date.toGMTString();
    document.cookie = name + '=' + value + '; ' + expires + '; path=/';
}
function getABTestContainer(el)
{
    if(el.parent().length === 0){
        return false;
    }
    if(el.hasClass('adsplaser_pro_abtest')){
        return el;
    }
    else {
        return getABTestContainer(el.parent());
    }
}

function AdsplacerProUtils() {

}

AdsplacerProUtils.prototype = {
    constructor: AdsplacerProUtils,
    isElementInView: function (element, fullyInView) {
        var pageTop = jQuery(window).scrollTop();
        var pageBottom = pageTop + jQuery(window).height();
        var elementTop = jQuery(element).offset().top;
        var elementBottom = elementTop + jQuery(element).height();

        if (fullyInView === true) {
            return ((pageTop < elementTop) && (pageBottom > elementBottom));
        } else {
            return ((elementTop <= pageBottom) && (elementBottom >= pageTop));
        }
    }
};
var AdsplacerProUtils = new AdsplacerProUtils();

window.adsplacerProAlreadyViewedAds = [];
window.adsplacerScrollTimeout = null;

function adsplacerViewAd(el){
    if(AdsplacerProUtils.isElementInView(jQuery(el), false) && jQuery.inArray(el, window.adsplacerProAlreadyViewedAds) === -1){
        window.adsplacerProAlreadyViewedAds[window.adsplacerProAlreadyViewedAds.length] = el;
        var container = jQuery(el);
        var ad_id = container.data('id');
        var template_id = container.data('template');
        var tag = container.data('tag');
        var p_number = container.data('p-number');
        var shortcode = container.data('shortcode');
        jQuery.ajax({
            url: '/wp-admin/admin-ajax.php',
            data: {
                action: 'ab_test_view',
                data : {
                    ad_id: ad_id,
                    template_id: template_id,
                    tag: tag,
                    shortcode: shortcode,
                    p_number: p_number
                }
            },
            type: 'POST'
        });
    }
}


function adsplacerTrackIframeClick(shortcodes)
{
    if(typeof shortcodes !== 'undefined' && shortcodes){
        shortcodes = '[data-shortcode]';
    }
    else {
        shortcodes = '';
    }
    jQuery('.adsplaser_pro_abtest' + shortcodes + ' iframe').iframeTracker({
        blurCallback: function(){
            var container = this._container;
            var ad_id = container.data('id');
            var template_id = container.data('template');
            var tag = container.data('tag');
            var p_number = container.data('p-number');
            var shortcode = container.data('shortcode');
            jQuery.ajax({
                url: '/wp-admin/admin-ajax.php',
                data: {
                    action: 'ab_test_click',
                    ad_id: ad_id,
                    template_id: template_id,
                    tag: tag,
                    shortcode: shortcode,
                    p_number: p_number
                },
                type: 'POST'
            });
        },
        overCallback: function(element, event) {
            this._container = jQuery(element).parents('.adsplaser_pro_abtest');
        },
        outCallback: function(element, event) {
            this._container = null;
        },
        _container: null
    });
}

function adsplacerTrackAdClick(shortcodes)
{
    if(typeof shortcodes !== 'undefined' && shortcodes){
        shortcodes = '[data-shortcode]';
    }
    else {
        shortcodes = '';
    }
    jQuery('.adsplaser_pro_abtest' + shortcodes).on('click', function(){
        var container = jQuery(this);
        var ad_id = container.data('id');
        var template_id = container.data('template');
        var tag = container.data('tag');
        var p_number = container.data('p-number');
        var shortcode = container.data('shortcode');
        jQuery.ajax({
            url: '/wp-admin/admin-ajax.php',
            data: {
                action: 'ab_test_click',
                ad_id: ad_id,
                template_id: template_id,
                tag: tag,
                shortcode: shortcode,
                p_number: p_number
            },
            type: 'POST'
        });
    });
}

function adsplacerInitExternalScripts()
{
    if(typeof adsplacerExternalFunctionsToExecute === 'object'){
        for(var i = 0; i < adsplacerExternalFunctionsToExecute.length;i++){
            if(typeof window[adsplacerExternalFunctionsToExecute[i]] === 'function'){
                window[adsplacerExternalFunctionsToExecute[i]]();
            }
        }
    }
}