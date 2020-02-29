var eduLevelList = new Array();
var topicList = new Array();
var prodIntegrationList = new Array();
var InstitutonSizeList = new Array();
var regionList = new Array();
var countryList = new Array();
var languageList = new Array();
var searchList = new Array();
var checkedItemList = new Array();
var lastClicked = "uhfSkipToMain";
var allRegions = ["africa", "asia", "caribbean", "europe", "latin america", "middle east", "north america", "oceania"];
var africanRegions = [];
var asianRegions = ["en-id", "en-ph", "en-sg", "id-id", "ja-jp", "ko-kr", "vi-vn", "zh-cn", "zh-hk", "zh-tw"];
var caribbeanRegions = [];
var europeRegions = ["cs-cz", "da-dk", "de-at", "de-ch", "de-de", "en-gb", "en-ie", "es-es", "fi-fi", "fr-be", "fr-fr", "hu-hu", "it-it", "nb-no", "nl-be", "nl-nl", "pl-pl", "pt-pt", "ro-ro", "ru-ru", "sv-se", "tr-tr", "uk-ua"];
var latinAmericanRegions = ["es-xl", "es-mx", "pt-br"];
var middleEasternRegions = ["ar-ae","ar-gulf", "ar-sa"];
var northAmericanRegions = ["en-ca", "en-us", "fr-ca"];
var oceanicRegions = ["en-au", "en-nz"];
var selectedRegion = "All regions";


window.addEventListener("load", function () {
    localStorage.setItem('state', 'closed');
});

function searchResults(event, currentLocale, currentPage, requestUrl) {
    var key = event.which || event.keyCode || e.key || 0;
    if (event.keyCode === $.ui.keyCode.ENTER) {
        event.preventDefault();
        var searchVal = $('#search-field').val();
        updateSelectedFiltersList('src', searchVal, searchVal, currentLocale, currentPage, requestUrl, false, false, event);
        $('#search-field').val("");
    }
}

function searchResultsClick(event, currentLocale, currentPage, requestUrl) {
    event.preventDefault();
    var searchVal = $('#search-field').val();
    updateSelectedFiltersList('src', searchVal, searchVal, currentLocale, currentPage, requestUrl, false, false, event);
    $('#search-field').val("");
}

function initCustomerStories(currentLocale, currentPage, requestUrl, isFirst) {

    if (isFirst == undefined || isFirst == "true") {
        localStorage.clear();
    }

    var filters = [],
        filterValue;
    filters.push({ key: "edulvl", value: eduLevelList });
    filters.push({ key: "Top", value: topicList });
    filters.push({ key: "reg", value: regionList });
    filters.push({ key: "ISIze", value: InstitutonSizeList });
    filters.push({ key: "prod", value: prodIntegrationList });
    filters.push({ key: "q", value: searchList });
    filters.push({ key: "reg", value: regionList });

    filters.forEach(function (filter) {
        filterValue = localStorage.getItem(filter.key);
        if (filterValue) {
            updateRangeFilterList(filter.value, filterValue);
        }
    });

    updateCustomerStories(currentLocale, currentPage, requestUrl, isFirst);
}

function expandDevices() {
    var hiddenContent = document.querySelector('#expanding-content');
    var targetState = localStorage.getItem('state');

    if (targetState === 'open') {
        hiddenContent.style.display = 'none';
        localStorage.setItem('state', 'closed');
    } else {
        hiddenContent.style.display = "block";
        localStorage.setItem('state', 'open');
    }

    document.querySelector("#DevicesFilterButton").classList.toggle("ctex-glyph-downArrow");
    document.querySelector("#DevicesFilterButton").classList.toggle("ctex-glyph-upArrow");

    document.querySelector(".showStories").classList.toggle("ctex-hidden");
    document.querySelector(".hideStories").classList.toggle("ctex-hidden");

}

function updateCustomerStories(currentLocale, currentPage, requestUrl, isFirst, e) {
    if (e != undefined) {
        e.preventDefault();
    }

    updateLocalStorage(currentLocale, currentPage);

    var requestParams = {
        edulvl: eduLevelList.join(","),
        Top: topicList.join(","),
        prod: prodIntegrationList.join(","),
        ISize: InstitutonSizeList.join(","),
        reg: regionList.join(","),
        clo: currentLocale,
        q: searchList.join(","),
        p: currentPage,
        i: isFirst,
        lang: currentLocale
    }
    $.ajaxSetup({
        cache: false
    });

    $.get(requestUrl, requestParams)

        .done(function (data) {

            if (isFirst) {
                testLocaleGet(currentLocale);
                localStorage.clear();
                checkedItemList = [];
                eduLevelList = [];
                topicList = [];
                regionList = [];
                prodIntegrationList = [];
                InstitutonSizeList = [];
                searchList = [];

                if (!currentLocale) {
                    selectedRegion = "All Regions";
                }
                else {
                    var test = selectedRegion.toLowerCase();
                    updateSelectedFiltersList('reg', selectedRegion, test, currentLocale, currentPage, requestUrl, true, false, e);
                }
            }

            $('#customer-stories-results').html(data);

            keepCheckedSelections();

            var hiddenContent = document.getElementById('expanding-content');
            var targetState = localStorage.getItem('state');
            if (targetState === 'open') {
                hiddenContent.style.display = 'block';
                document.querySelector(".showStories").classList.add("ctex-hidden");
                document.querySelector(".hideStories").classList.remove("ctex-hidden");
                document.querySelector("#DevicesFilterButton").classList.remove("ctex-glyph-downArrow");
                document.querySelector("#DevicesFilterButton").classList.add("ctex-glyph-upArrow");
                localStorage.setItem('state', 'open');
            }
            else {
                hiddenContent.style.display = "none";
                document.querySelector(".showStories").classList.remove("ctex-hidden");
                document.querySelector(".hideStories").classList.add("ctex-hidden");
                document.querySelector("#DevicesFilterButton").classList.add("ctex-glyph-downArrow");
                document.querySelector("#DevicesFilterButton").classList.remove("ctex-glyph-upArrow");
                localStorage.setItem('state', 'closed');
            }
        })

        .fail(function () {
            console.log('Failed request');
        })

        .always(function () {
            /* Set focus to the first result container / no results message.
               Remove it once user interacts with the page to resume normal MWF behaviour.*/
            var firstResult = null;
            if (!isFirst) {
                firstResult = $('#customer-stories-results #rightPanel .m-content-placement-item div').first();
                if (firstResult.length === 0) {
                    firstResult = $('#customer-stories-results #rightPanel .no-results').first();
                }
                firstResult.attr('tabindex', '0');
                firstResult.focus();
                firstResult.on('blur', function () {
                    $(this).removeAttr('tabindex');
                });
            }
        });
}

function keepCheckedSelections() {

    checkedItemList.forEach(function (listItem) {
        var checkSelection = document.getElementById(listItem);
        lastClicked = checkedItemList[checkedItemList.length - 1];

        if (checkSelection) {
            checkSelection.setAttribute('checked', 'checked');
        }
    });

    document.getElementById("CurrentId").innerHTML = selectedRegion.toString();
}

function updateSelectedFiltersList(from, categName, id, currentLocale, currentPage, requestUrl, isFirst, button, e) {
    var index = checkedItemList.indexOf(id);

    Array.prototype.contains = function (needle) {
        for (i in this) {
            if (this[i] == needle) return true;
        }
        return false;
    };
    if (index > -1) {
        checkedItemList.splice(index, 1);
    }
    else {
        checkedItemList.push(id);
    }
    if (allRegions.contains(id)) {
        selectedRegion = categName;
    }

    if (button == true) {
        if (allRegions.contains(id)) {
            selectedRegion = "All Regions";
        }
    }

    if (e != undefined) {
        e.preventDefault();
    }

    lastClickedDrawerItemId = id;
    categName = decodeURIComponent(categName);

    if (button === true) {
        LastClicked = document.querySelector("body " + "#clearAllFilters");
    }
    else {
        LastClicked = document.querySelector("body " + "#remove" + id);
    }

    setTimeout(function () {
        var LastClicked = document.querySelector("body " + "#" + id);
        LastClicked.focus();
    }, 200);

    if (from === 'edulvl') {
        updateFilterList(eduLevelList, categName);
    }
    else if (from === 'Top') {
        updateFilterList(topicList, categName);
    }
    else if (from === 'ISize') {
        updateFilterList(InstitutonSizeList, categName);
    }
    else if (from === 'prod') {
        updateFilterList(prodIntegrationList, categName);
    }
    else if (from == 'src') {
        updateFilterList(searchList, categName);
    }
    else if (from == 'reg') {
        updateFilterList(regionList, categName);

    }

    var hiddenContent = document.getElementById('expanding-content');
    var targetState = localStorage.getItem('state');
    if (!isFirst) {
        if (targetState === 'open') {
            hiddenContent.style.display = 'block';
            localStorage.setItem('state', 'open');
        }
        else {
            hiddenContent.style.display = "none";
            localStorage.setItem('state', 'closed');
        }
    }

    updateCustomerStories(currentLocale, currentPage, requestUrl, false);
}

function updateLocalStorage(currentLocale, currentPage) {
    if (typeof (mscc) == 'undefined' || !mscc || mscc.hasConsent()) {
        localStorage.setItem("edulvl", escapeHtml(eduLevelList.join(",")));
        localStorage.setItem("Top", escapeHtml(topicList.join(",")));
        localStorage.setItem("prod", escapeHtml(prodIntegrationList.join(",")));
        localStorage.setItem("ISize", escapeHtml(InstitutonSizeList.join(",")));
        localStorage.setItem("reg", escapeHtml(regionList.join(",")));
        localStorage.setItem("q", escapeHtml(searchList.join(",")));
        localStorage.setItem("p", currentPage);
    }
}

function updateFilterList(currList, itemName) {
    if ($.inArray(itemName, currList) === -1) {
        currList.push(itemName);
    } else {
        removefromList(currList, itemName);
    }
}

function updateRangeFilterList(currList, itemList) {
    var array = itemList.split(',');
    for (var i = 0; i < array.length; i++) {
        updateFilterList(currList, array[i]);
    }
}

function removefromDict(dict, id) {
    for (var i in dict) {
        if (dict[i].key == id) {
            delete dict[i];
        }
    }
}

function IsInDictionary(dict, id) {
    for (var i in dict) {
        if (dict[i].key == id) {
            return true;
        }
    }
    return false;
}

function removefromList(arr, item) {
    for (var i = arr.length; i--;) {
        if (arr[i] === item) {
            arr.splice(i, 1);
        }
    }
}
function clearAllFilters(currentLocale, currentPage, requestUrl) {
    checkedItemList.forEach(function (listItem) {
        var checkSelection = document.getElementById(listItem);
        if (checkSelection) {
            checkSelection.removeAttribute('checked');
        }
    });

    checkedItemList = [];
    eduLevelList = [];
    topicList = [];
    prodIntegrationList = [];
    InstitutonSizeList = [];
    regionList = [];
    searchList = [];

    selectedRegion = "All regions";

    updateCustomerStories(currentLocale, currentPage, requestUrl, false);
}

var entityMap = {
    "&": "-",
    "<": "-",
    ">": "-",
    '"': '-',
    "'": '-',
    "/": '-',
    " ": '-'
};

function escapeHtml(string) {
    return String(string).replace(/[&<>"'\/]/g, function (s) {
        return entityMap[s];
    });
}

//Resize divs so they are all the same height

function resizeContainers() {
    $('.item-container').height("auto");

    $('[id^="row-"]').each(function () {
        var highestBox = 0;
        $('.item-container', this).each(function () {
            if ($(this).height() > highestBox) {
                highestBox = $(this).height();
            }
        });
        $('.item-container', this).height(highestBox + 80);
    });
}

//Set up initial filters from current locale
function testLocaleGet(currentLocale) {
    Array.prototype.contains = function (IsInArray) {
        for (i in this) {
            if (this[i] == IsInArray) return true;
        }
        return false;
    };
    if (africanRegions.contains(currentLocale)) {
        selectedRegion = "Africa";
    }
    else if (asianRegions.contains(currentLocale)) {
        selectedRegion = "Asia";
    }
    else if (caribbeanRegions.contains(currentLocale)) {
        selectedRegion = "Caribbean";
    }
    else if (europeRegions.contains(currentLocale)) {
        selectedRegion = "Europe";
    }
    else if (latinAmericanRegions.contains(currentLocale)) {
        selectedRegion = "Latin America";
    }
    else if (middleEasternRegions.contains(currentLocale)) {
        selectedRegion = "Middle East";
    }
    else if (northAmericanRegions.contains(currentLocale)) {
        selectedRegion = "North America";
    }
    else if (oceanicRegions.contains(currentLocale)) {
        selectedRegion = "Oceania";
    }
    else if (!currentLocale) {
        selectedRegion = "";
    }
}
