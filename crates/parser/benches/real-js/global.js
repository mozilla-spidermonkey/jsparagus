function showtab(tab) {
    document.querySelector('#tab-' + tab).click();
}

$('#focustop').on('click', function () {
    // when back to the top is clicked the focus returns to tab-leadership
    $('#tab-leadership').focus();
});

function scrollWin() {
    window.scrollTo(0, 0)
}

function carouselevents() {
    var indicator, firstIndicator, styles;

    $('.c-carousel[data-ctex-dynamic-colour] .c-sequence-indicator button').on('focus', function () {
        indicator = $(this).parents('.c-sequence-indicator').first();
        indicator.removeClass();
        styles = $(this).attr('class');
        indicator.addClass('c-sequence-indicator').addClass(styles);
    });

    $('.c-carousel[data-ctex-dynamic-colour]').each(function (index, element) {
        firstIndicator = $(element).find('.c-sequence-indicator button').first();
        styles = firstIndicator.attr('class');
        $(element).find('.c-sequence-indicator').addClass(styles);
    });

    $('.c-carousel[data-ctex-dynamic-colour] .c-sequence-indicator button > span').on('click', function () {
        $(this).parent().trigger('click');
    });
}

function DeviceFinderInit() {
    var thisElementId, availableFeatures, featureSupported, tooltipId = 0;

    /* automatically assign tooltips with their associated controls */
    $('.ctex-device-kiosk [data-toggle="tooltip"]').each(function (index) {
        thisElementId = 'tooltip-icon-strip-' + index;
        $(this).removeAttr('aria-describedby').attr('aria-describedby', thisElementId);
        $(this).next('.c-tooltip').removeAttr('id').attr('id', thisElementId);
        $(this).find('.unavailable, .available').hide();
    });

    $('.ctex-device-kiosk .ctex-icon-strip').each(function (index) {
        availableFeatures = parseInt($(this).data('features'));
        $(this).find('[data-toggle="tooltip"]').each(function (index) {
            thisElementId = 'tooltip-icon-strip-' + tooltipId;
            featureSupported = parseInt($(this).data('feature'));
            // only enable features relevant to the device
            if (featureSupported & availableFeatures) {
                $(this).attr('aria-describedby', thisElementId);
                $(this).next('.c-tooltip').removeAttr('id').attr('id', thisElementId);
                $(this).find('.available').show();
            }
            else {
                // ensure keyboard navigation does not gain focus & tooltip disabled
                $(this).attr('aria-describedby', thisElementId);
                $(this).next('.c-tooltip').removeAttr('id').attr('id', thisElementId);
                $(this).find('.unavailable').show();
            }
            $(this).on('click', function (e) {
                e.preventDefault();
            });
            tooltipId++;
        });
    });
}

function AutoAssignTooltips(el) {
    $(el).find('[data-toggle="tooltip"]').each(function (index) {
        thisElementId = 'tooltip' + index;
        $(this).removeAttr('aria-describedby');
        if (!$.trim($(this).next('.c-tooltip').text())) {
            $(this).next('.c-tooltip').remove();
            $(this).removeAttr('data-toggle');
        }
        else {
            $(this).attr('aria-describedby', thisElementId);
            $(this).next('.c-tooltip').removeAttr('id').attr('id', thisElementId);
        }
    });
}

$(document).ready(function () {
    carouselevents();
});

/* Inline YouTube control verical centring logic */

function sizeYTContainers() {
    var maxHeight = 0;
    $($('.ctex-youtube-inline').find('.f-y-center[data-grid~="col-6"]')).each(function (index) {
        if (elementIsStacked($(this).parents('[data-grid*="stack-"]').first())) {
            maxHeight = 'auto';
        }
        else {
            maxHeight = $(this).height() > maxHeight ? $(this).height() : maxHeight;

            if ($(this).hasClass('ctex-youtube-inline-padding')) {
                maxHeight = maxHeight + 50;
            }
        }
    }).parent().height(maxHeight);
}

$(window).resize(function () {
    clearTimeout(window.resizedFinished);
    window.resizedFinished = setTimeout(function () {
        sizeYTContainers();
    }, 50);
});

function elementIsStacked(el) {
    var result = false, windowWidth = window.innerWidth;
    result = result || (windowWidth <= 539 && $(el).filter('[data-grid*="stack-1"]').length > 0);
    result = result || (windowWidth <= 767 && $(el).filter('[data-grid*="stack-2"]').length > 0);
    result = result || (windowWidth <= 1083 && $(el).filter('[data-grid*="stack-3"]').length > 0);
    result = result || (windowWidth <= 1399 && $(el).filter('[data-grid*="stack-4"]').length > 0);
    result = result || (windowWidth <= 1778 && $(el).filter('[data-grid*="stack-5"]').length > 0);
    result = result || (windowWidth <= 1779 && $(el).filter('[data-grid*="stack-6"]').length > 0);
    return result;
}

$(document).ready(function () {
    sizeYTContainers();
});

/* END Inline YouTube control verical centring logic */

//Disable LP Chat default action to avoid link click moving you to the top of the page
$('.lpchat').click(function (e) {
    e.preventDefault;
});

function isInt(n) {
    return typeof n === 'number' && n % 1 == 0;
}

/*
    Will continue to poll callback function every 'pollInterval' ms a maximum of 'retries' times.
    If the callback function returns true, it stops further polls.
*/
function retryWithMaxAttempts(callback, pollInterval, retries) {
    if (retries > 0) {
        window.setTimeout(function () {
            if (callback()) {
                retries = 0;
            }
            retries--;
            retryWithMaxAttempts(callback, pollInterval, retries);
        }, pollInterval);
    }
}

function setPivotActiveTab(pivotCollection, pivotId, pivotTabIndex, sectionId) {
    if (pivotId != null) {
        $(pivotCollection).each(function (idx, v) {
            if (isInt(pivotTabIndex) && $(pivotCollection[idx].element).attr('id') == pivotId) {
                pivotCollection[idx].pivotTabs[pivotTabIndex].click();
                if (sectionId != null && sectionId.length > 0) {
                    retryWithMaxAttempts(function () {
                        if ($('#' + sectionId).is(':visible')) {
                            window.setTimeout(function () {
                                $(document).scrollTop($('#' + sectionId).offset().top);
                            }, 500);
                            return true;
                        }
                        return false;
                    }, 200, 20);
                }
            }
        });
    }
}

function getCountryByLocale(locale) {

    switch (locale) {
        case "en-au": return "Australia";
        case "nl-be": return "Belgium";
        case "fr-be": return "Belgium";
        case "pt-br": return "Brazil";
        case "en-ca": return "Canada";
        case "fr-ca": return "Canada";
        case "es-cl": return "Chile";
        case "zh-cn": return "China";
        case "es-co": return "Colombia";
        case "cs-cz": return "Czech Republic";
        case "da-dk": return "Denmark";
        case "es-ec": return "Ecuador";
        case "fi-fi": return "Finland";
        case "fr-fr": return "France";
        case "de-at": return "Austria";
        case "de-de": return "Germany";
        case "en-gb": return "United Kingdom";
        case "ar-ae": return "Gulf";
        case "en-in": return "India";
        case "en-id": return "Indonesia";
        case "en-ie": return "Ireland";
        case "it-it": return "Italy";
        case "ja-jp": return "Japan";
        case "ko-kr": return "South Korea";
        case "es-mx": return "Mexico";
        case "nl-nl": return "Netherlands";
        case "en-nz": return "New Zealand";
        case "nb-no": return "Norway";
        case "es-pe": return "Peru";
        case "en-ph": return "Philippines";
        case "pt-pt": return "Portugal";
        case "ru-ru": return "Russia";
        case "ar-sa": return "Saudi Arabia";
        case "en-sg": return "Singapore";
        case "es-es": return "Spain";
        case "sv-se": return "Sweden";
        case "de-ch": return "Switzerland";
        case "zh-tw": return "Taiwan";
        case "tr-tr": return "Turkey";
        case "uk-ua": return "Ukraine";
        case "en-us": return "United States";
        case "vi-vn": return "Vietnam";

        default: return "United States";
    }
}

function ApplyPivotFixes(pivotCollection) {
    $(pivotCollection).each(function (idx, v) {
        v.subscribe({
            onPivotChanged: function (notification) {
                $(pivotCollection).each(function (idx, v) {
                    /*
                    Workaround for MWF 1.25.1 issue.
                    Flippers behaviour based upon calculated size of Pivot header which
                    is called at instantiation phase only.
                    If a flipper isn't visible on page load (e.g. a sub-pivot),
                    then the calculations will be wrong resulting in flippers
                    behaving erratically when the hidden Pivot becomes visible.
                    This workaround forces a recalculate of the calculations
                    for every Pivot control on the page if the state of one
                    of them changes so we can take into account Pivots that
                    have just become visible.
                    */
                    v.initializeWidths();
                    v.setTabView();
                    v.updateFlippers();
                });
            }
        });
    });
}

/* Feature content bleed fix
    MWF m-feature requires image to alwasy be taller than accompanying text.
    Depending on locale and content, this is not always the case.
    Due to CSS3 transforms being used to provide vertical alignment,
    we cannot fix content bleed in CSS alone. Hence the following.
*/
function fixFeatureBleed() {
    var maxHeight = 0,
        featureIsStacked,
        currentWidth = window.innerWidth,
        stackBreakpoint,
        elemHeight = 0;

    $('.m-feature.ctex-feature-bleed-fix, .c-feature.ctex-feature-bleed-fix').each(function (indexFeature) {
        var isVideo = false;

        if ($(this).hasClass("f-align-center")) {
            return;
        }

        // to be used post BETT
        // this was added as all c-feature have a fake stack break at stack-3 (1084) in MWFOverrides - temporary until c-feature is replaced with m-feature.
        //stackBreakpoint = getStackBreakpoint(this);

        if ($(this).hasClass('c-feature')) {
            stackBreakpoint = 1084;
        }
        else if ($(this).hasClass('m-feature')) {
            stackBreakpoint = 768;
        }

        maxHeight = 0;

        if ($(this).find('.m-ambient-video').height() != null) {
            elemHeight = $(this).find('.m-ambient-video').height();
            isVideo = true;
        }
        else {
            elemHeight = $(this).children('picture').find('img').height();
        }

        var contentHeight = $(this).children('div:not(.m-ambient-video)').height();

        maxHeight = elemHeight < contentHeight ? contentHeight : elemHeight;

        $(this).find('picture img, .m-ambient-video').removeClass('force-vertical-align').addClass('force-vertical-align');

        $(this).find('picture').css({
            'height': 'inherit',
            'position': 'relative'
        });

        featureIsStacked = currentWidth < stackBreakpoint ? true : false;

        $(this).children().each(function (childIndex) {
            if (featureIsStacked) {
                maxHeight = '100%';
                if (isVideo) {
                    $(this).removeClass('force-vertical-align');
                }
                else {
                    $(this).find('img').removeClass('force-vertical-align');
                }
            }
        }).parent().height(maxHeight);
    });
}

/**
 * Fix for MWF dialog component to ensure aria-hidden is applied to background elements when a dialog is opened.
 */
function dialogFix() {
    $('main, section').attr('data-js-controlledby', 'dialog');
}

// to be used post BETT
//function getStackBreakpoint(element) {
//    var result;

//    if ($(element).hasClass('stack-1')) {
//        result = 539;
//    }
//    else if ($(element).hasClass('stack-2')) {
//        result = 767;
//    }
//    else if ($(element).hasClass('stack-3')) {
//        result = 1083;
//    }
//    else if ($(element).hasClass('stack-4')) {
//        result = 1399;
//    }
//    else if ($(element).hasClass('stack-5')) {
//        result = 1778;
//    }
//    else if ($(element).hasClass('stack-6')) {
//        result = 1779;
//    }
//    return result;
//}

window.onload = function () {
    fixFeatureBleed();
};

var resizeId;

$(window).resize(function () {
    clearTimeout(resizeId);
    resizeId = setTimeout(doneResizing, 100);
});

function doneResizing() {
    fixFeatureBleed()
}

// VFI AllY  fix for iPad/Android etc.
$('.m-social > ul > li > a, .c-call-to-action, .c-action-trigger, .c-hyperlink').each(function (index, el) {
    $(el).addClass('ctex-tap-disable');
});
document.addEventListener("touchstart", function () { }, true);

//Drawer toggle for Cortex One US Office page
$(document).ready(function () {
    officeFAQDrawers();
});

function officeFAQDrawers() {

    $('.officeFAQDrawers .c-drawer > div').hide();

    $('.officeFAQDrawers .c-drawer button').on('click', function (ev) {
        if (ev.which == 1) {
            $(this).parent().find('div:last-of-type').slideToggle('slow');
            if ($(this).attr('aria-expanded') == 'false') {
                $(this).attr('aria-expanded', 'true');
            }
            else {
                $(this).attr('aria-expanded', 'false');
            }
        }
    });

}

//Social Footer//
var DescString = $("meta[property='og:description']").attr("content");
var ImageLocation = $("meta[property='og:image']").attr("content");
var pathName = window.location.pathname;
var UrlStringTwitter = "http://twitter.com/share?url=https://www.microsoft.com/" + pathName + "&amp;text=" + DescString + " @MicrosoftEDU";
var UrlStringFacebook = "https://www.facebook.com/sharer/sharer.php?u=https://www.microsoft.com/" + pathName;
var UrlStringLinkedIn = "https://www.linkedin.com/shareArticle?mini=true&url=https://www.microsoft.com/" + pathName;
var UrlPintrest = "https://www.pinterest.com/pin/create/button/?url=https://www.microsoft.com/" + pathName + "&media=" + ImageLocation + "&description=" + DescString;

document.getElementById("ShareTwitter").setAttribute("data-href", UrlStringTwitter);
document.getElementById("ShareFB").setAttribute("data-href", UrlStringFacebook);
document.getElementById("ShareLinkedIn").setAttribute("data-href", UrlStringLinkedIn);
document.getElementById("SharePintrest").setAttribute("data-href", UrlPintrest);
window.fbAsyncInit = function () {
    FB.init({
        appId: '171047642947607', status: true, cookie: true, xfbml: true
    });
};

(function (d, s, id) {
    var js, fjs = d.getElementsByTagName(s)[0];
    if (d.getElementById(id)) return;
    js = d.createElement(s); js.id = id;
    js.src = 'https://connect.facebook.net/en_GB/sdk.js#xfbml=1&version=v3.1&appId=1922676434441906&autoLogAppEvents=1';
    fjs.parentNode.insertBefore(js, fjs);
}(document, 'script', 'facebook-jssdk'));

$('#ShareFB').click(function () {
    elem = $(this);
    postToFeed(elem.data('title'), elem.data('desc'), elem.prop('href'), elem.data('image'));
    return false;
});

