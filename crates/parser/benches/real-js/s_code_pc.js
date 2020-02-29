// --- Initialize ---------------------------
if (typeof YAHOO === "undefined") {
    var YAHOO = {};
    YAHOO.JP = {};
    YAHOO.JP.shp = {};
} else if (typeof YAHOO.JP === "undefined") {
    YAHOO.JP = {};
    YAHOO.JP.shp = {};
} else if (typeof YAHOO.JP.shp === "undefined") {
    YAHOO.JP.shp = {};
}

// --- Preference object ---------------------
YAHOO.JP.AA_PREF = {};
// view type "PC" setting
YAHOO.JP.AA_PREF.pc = {
    // Adobe Analytics common preference
    general: {
        linkInternalFilters: "javascript:,beststore.yahoo.co.jp,coupon.shopping.yahoo.co.jp,docs.yahoo.co.jp,login.yahoo.co.jp,order.yahoo.co.jp,otoriyose.yahoo.co.jp,points.yahoo.co.jp,present.crocos.jp,review.store.shopping.yahoo.co.jp,shopping.geocities.jp,search.shopping.yahoo.co.jp,shopping.yahoo.co.jp,starclub.yahoo.co.jp,store-events.yahoo.co.jp,www.fukko-department.jp,info.shinsai.yahoo.co.jp,shopping.search.yahoo.co.jp,ord.yahoo.co.jp,order.store.yahoo.co.jp,yimg.jp,nassertool.shopping.yahoo.co.jp,ck.storematch.jp,stage.storematch.jp,approach.yahoo.co.jp,app.commerce21.co.jp,talk.shopping.yahoo.co.jp"
    },
    // Y!Shopping original preference
    specific: {
        tracking_ad_pattern: ["ck.storematch.jp","stage.storematch.jp"],
        campaign_matching: ["topics.shopping.yahoo.co.jp", "contents.shopping.yahoo.co.jp", "shopping.yahoo.co.jp/fair/","sales.shopping.yahoo.co.jp", "shopping.yahoo.co.jp/event/","www.fukko-department.jp","info.shinsai.yahoo.co.jp"],
        siteID: {
            "beststore.yahoo.co.jp": "beststore",
            "category.shopping.yahoo.co.jp": "category",
            "contents.shopping.yahoo.co.jp": "contents",
            "coupon.shopping.yahoo.co.jp": "coupon",
            "docs.yahoo.co.jp": "docs",
            "login.yahoo.co.jp": "login",
            "order.shopping.yahoo.co.jp": "order",
            "order.store.yahoo.co.jp": "store",
            "otoriyose.yahoo.co.jp": "otoriyose",
            "odhistory.shopping.yahoo.co.jp": "odhistory",
            "review.store.shopping.yahoo.co.jp": "review",
            "shopping.geocities.jp": "triple",
            "search.shopping.yahoo.co.jp": "search",
            "shopping.search.yahoo.co.jp": "search",
            "shopping.yahoo.co.jp": "main",
            "starclub.yahoo.co.jp": "starclub",
            "store-events.yahoo.co.jp": "store-events",
            "store.shopping.yahoo.co.jp": "store",
            "tpoint.shopping.yahoo.co.jp": "tpoint",
            "topics.shopping.yahoo.co.jp": "topics",
            "sales.shopping.yahoo.co.jp": "sales",
            "www.yahoo-help.jp": "help",
            "www.fukko-department.jp": "fukko-department",
            "info.shinsai.yahoo.co.jp": "info.sinsai",
            "adult.shopping.yahoo.co.jp": "main.adult",
            "search.adult.shopping.yahoo.co.jp": "search.adult",
            "store.adult.shopping.yahoo.co.jp": "store.adult",
            "mail.shopping.yahoo.co.jp": "mail",
            "snlweb.shopping.yahoo.co.jp": "snlweb",
            "inform.shopping.yahoo.co.jp": "inform",
            "talk.shopping.yahoo.co.jp": "talk"
        }
    }
};

// view type "SP" Setting
YAHOO.JP.AA_PREF.sp = {
    // Adobe Analytics common preference
    general: {
        linkInternalFilters: "javascript:,beststore.yahoo.co.jp,coupon.shopping.yahoo.co.jp,docs.yahoo.co.jp,login.yahoo.co.jp,order.yahoo.co.jp,otoriyose.yahoo.co.jp,points.yahoo.co.jp,present.crocos.jp,review.store.shopping.yahoo.co.jp,shopping.geocities.jp,search.shopping.yahoo.co.jp,shopping.yahoo.co.jp,starclub.yahoo.co.jp,store-events.yahoo.co.jp,www.fukko-department.jp,info.shinsai.yahoo.co.jp,shopping.search.yahoo.co.jp,ord.yahoo.co.jp,order.store.yahoo.co.jp,yimg.jp,nassertool.shopping.yahoo.co.jp,ck.storematch.jp,stage.storematch.jp,approach.yahoo.co.jp,app.commerce21.co.jp,talk.shopping.yahoo.co.jp"
    },
    // Y!Shopping original preference
    specific: {
        tracking_ad_pattern: ["ck.storematch.jp","stage.storematch.jp"],
        campaign_matching: ["topics.shopping.yahoo.co.jp", "contents.shopping.yahoo.co.jp", "shopping.yahoo.co.jp/fair/","sales.shopping.yahoo.co.jp", "shopping.yahoo.co.jp/event/","www.fukko-department.jp","info.shinsai.yahoo.co.jp"],
        siteID: {
            "beststore.yahoo.co.jp": "sp_beststore",
            "coupon.shopping.yahoo.co.jp": "sp_coupon",
            "docs.yahoo.co.jp": "sp_docs",
            "login.yahoo.co.jp": "sp_login",
            "m.yahoo-help.jp": "sp_help",
            "m.shopping.yahoo.co.jp": "sp_main",
            "order.shopping.yahoo.co.jp": "sp_order",
            "order.store.yahoo.co.jp": "sp_store",
            "otoriyose.yahoo.co.jp": "sp_otoriyose",
            "review.store.shopping.yahoo.co.jp": "sp_review",
            "shopping.geocities.jp": "sp_triple",
            "search.shopping.yahoo.co.jp": "sp_search",
            "shopping.search.yahoo.co.jp": "sp_search",
            "shopping.yahoo.co.jp": "sp_main",
            "starclub.yahoo.co.jp": "sp_starclub",
            "store-events.yahoo.co.jp": "sp_store-events",
            "store.shopping.yahoo.co.jp": "sp_store",
            "category.shopping.yahoo.co.jp": "sp_category",
            "contents.shopping.yahoo.co.jp": "sp_contents",
            "odhistory.shopping.yahoo.co.jp": "sp_odhistory",
            "smartphone.shopping.yahoo.co.jp": "sp_smartphone",
            "tpoint.shopping.yahoo.co.jp": "sp_tpoint",
            "sales.shopping.yahoo.co.jp": "sp_sales",
            "topics.shopping.yahoo.co.jp": "sp_topics",
            "www.fukko-department.jp": "sp_fukko-department",
            "info.shinsai.yahoo.co.jp": "sp_info.sinsai",
            "adult.shopping.yahoo.co.jp": "sp_main.adult",
            "search.adult.shopping.yahoo.co.jp": "sp_search.adult",
            "store.adult.shopping.yahoo.co.jp": "sp_store.adult",
            "mail.shopping.yahoo.co.jp": "sp_mail",
            "snlweb.shopping.yahoo.co.jp": "sp_snlweb",
            "inform.shopping.yahoo.co.jp": "sp_inform",
            "talk.shopping.yahoo.co.jp": "sp_talk"
        }
    }
};


(function() {

    var s_account = "yahoojapanshoppingdev";
    var s = s_gi(s_account);

    // s_code version(for set prop75)
    // IMPORTANT: if fixed this file, please modify "s.code_version" !
    // RULE: yyyy-M-dd
    s.code_version = "AM" + s.version + "_2019-04-11";

    // setting for change from s_code to AppMeasurement
    s.wd=window;

    /************************** CONFIG SECTION **************************/

    s.charSet = "UTF-8";
    s.cookieDomainPeriods = "4";
    s.fpCookieDomainPeriods = "4";
    s.currencyCode = "JPY";
    s.trackDownloadLinks = false;
    s.trackExternalLinks = true;
    // this parameter is to be overriden.
    s.linkInternalFilters = "javascript:,shopping.yahoo.co.jp";
    s.linkLeaveQueryString = true;
    s.linkTrackVars = "None";
    s.linkTrackEvents = "None";
    s.usePlugins = true;
    s.linkInternalFilters += "," + location.host;

    /* Page Name Config */
    s.server = s.server || s.wd.location.hostname;
    s.channel = "shp";
    s.eVar1 = "D=channel";
    s.subDomLst = {};

    // getPageName plugin setting
    s.defaultPage = "index";
    s.queryVarsList = "";
    s.pathExcludeDelim = ".";
    s.pathConcatDelim = ":";
    s.pathExcludeList = "cgi-bin";

    // Overwrite mid from cookie
    s.mcid = s.Util.cookieRead("mcid");
    var isApp = window.navigator && window.navigator.userAgent && window.navigator.userAgent.indexOf("yshopping") > -1;
    var isVid = YAHOO && YAHOO.JP && YAHOO.JP.shp && YAHOO.JP.shp.data && YAHOO.JP.shp.data.status && YAHOO.JP.shp.data.status.visitorID != null;
    if(s.mcid){
        // Experience Cloud ID (mid) Service
        s.visitor = Visitor.getInstance("257D34B852785CAF0A490D44@AdobeOrg", {
                        trackingServer: "yahoojapanshopping.sc.omtrdc.net",
                        overwriteCrossDomainMCIDAndAID: true,
                        cookieDomain:"shopping.yahoo.co.jp"
                    });
        if (s.visitor) {
            s.visitor.setMarketingCloudVisitorID(s.mcid);
        }
    }else if(!isApp || !isVid){
        // Experience Cloud ID (mid) Service
        s.visitor = Visitor.getInstance("257D34B852785CAF0A490D44@AdobeOrg", {
                        trackingServer: "yahoojapanshopping.sc.omtrdc.net",
                        overwriteCrossDomainMCIDAndAID: true,
                        cookieDomain:"shopping.yahoo.co.jp"
                    });
    }
    
    function s_doPlugins(s) {
        if(s.visitor){ s.mcid = s.mcid ? s.mcid : s.visitor.getMarketingCloudVisitorID();}
        // set s.eo and s.lnk object
        s.eo=s.lnk="";
        if(s.clickObject) {s.eo=s.clickObject;}

        // --- Initialize setting --------------------
        s.server = s.getLocation.hostname();
        s.channel = "shp";

        // Link(reference) pagedata Object
        if ("shp" in YAHOO.JP && "data" in YAHOO.JP.shp) {
            s.aa_data = YAHOO.JP.shp.data;
        } else {
            s.aa_data = {
                status: {}
            };
        }

        // irregular. from 2017/02/16. for campaign
        if ("shp" in YAHOO.JP && "data_exception" in YAHOO.JP.shp) {
            s.aa_data.exception = YAHOO.JP.shp.data_exception;
        }

        // Link(reference) Preference Setting Object
        // if viewType is not setting, default is "PC".
        // if device is not "pc", require section prefix.
        // device variable
        //   1: pc
        //   2: mobile
        //   3: sp
        //   4: tablet
        if (s.aa_data.status.device == 1) {
            s.sectionname_prefix = "";
            s.aa_pref = YAHOO.JP.AA_PREF.pc;
        } else if (s.aa_data.status.device == 3) {
            s.sectionname_prefix = "sp";
            s.aa_pref = YAHOO.JP.AA_PREF.sp;
        } else {
            s.sectionname_prefix = "";
            s.aa_pref = YAHOO.JP.AA_PREF.pc;
        }
        var un_prefix = "yahoojapanshopping";
        var un_suffix = "prd";
        var app_type = null;
        s.sa(un_prefix + s.sectionname_prefix + un_suffix);

       // include s_code_conf data.
        if ('general' in s.aa_pref) {
            // override general section
            s.linkInternalFilters = s.aa_pref.general.linkInternalFilters;
        }
        if ('specific' in s.aa_pref) {
            s.subDomLst = s.aa_pref.specific.siteID;
        }

        if (s.subDomLst[s.server]) {
            //switch reportsuitesp for SP app.
            if(window.navigator && window.navigator.userAgent && window.navigator.userAgent.indexOf("yshopping") > -1){
                if(window.navigator.userAgent.indexOf("Android") > -1){
                    s.sa(un_prefix + "androidapp" + un_suffix);
                    app_type = "android";
                }else{
                    s.sa(un_prefix + "iosapp" + un_suffix);
                    app_type = "ios";
                }
            }else{
                s.sa(un_prefix + s.sectionname_prefix + un_suffix);
            }
            s.subDomID = s.subDomLst[s.server];
        } else {
            if (s.wd.location.host.indexOf(".yahoo.co.jp") > -1) {
                s.subDomID = s.wd.location.host.replace(/\.yahoo\.co\.jp$/, "");
            } else {
                s.subDomID = "other";
            }
        }
        if (s.subDomID.indexOf("talk") > -1) {
            if (s.getLocation.pathname().indexOf("/contact/") == 0) {
                s.subDomID = s.aa_data.status.device != 3 ? "store": "sp_store";
            }
            else if (s.getLocation.pathname().indexOf("/detail/") == 0) {
                s.subDomID = s.aa_data.status.device != 3 ? "store": "sp_store";
            }
        }
        s.aa_pref.subDomID = s.subDomID;
        s.siteID = s.channel + ":" + s.subDomID;

        // [visitorId] visitor ID
        // ---------------------------------------------------------
        if (!s.mcid && "visitorID" in s.aa_data.status && s.aa_data.status.visitorID) {
          s.visitorID = s.aa_data.status.visitorID;
        }

        // --- Switch RSID (Access through AndroidApp.) --------------------
        // if Cookie "sc_device" is undefined -> PC
        // if Cookie "sc_device" is defined
        //   -> androidapp then androidapp
        //   -> sp then sp
        //   if s.subDomLst[s.server] is defined
        //     -> then production
        //     -> else test
        if (s.getQueryParam("vid") && s.getQueryParam("sc_device")) {
            s.c_w("sc_device", s.getQueryParam("sc_device"));
        }
        var sc_device = s.c_r("sc_device");
        var rsid = "yahoojapanshopping";
        if (sc_device) {
            switch (sc_device) {
                case "androidapp":
                    rsid += "androidapp";
                    break;
                default:
                    rsid += "sp";
                    break;
            }
            if (s.subDomLst[s.server]) {
                rsid += "prd";
            } else {
                rsid += "dev";
            }
            s.sa(rsid);
        }

        // @param {Array} a_referrer_url referrerURL Object(scheme/domain/port/path/query/hash)
        var a_referrer_url = s.getReferrer().match(/^(?:([^:\/?#]+):)?(?:\/\/([^\/?#]*))?([^?#]*)(?:\?([^#]*))?(?:#(.*))?/);
        
        if(a_referrer_url[0].indexOf("shopping.yahoo.co.jp/coupon") > 0) {
            a_referrer_url[2] = "coupon.shopping.yahoo.co.jp";
        } else if(a_referrer_url[0].indexOf("shopping.yahoo.co.jp/search") > 0) {
            a_referrer_url[2] = "search.shopping.yahoo.co.jp";
        }
        
        if(a_referrer_url.length >= 3 && s.subDomLst[a_referrer_url[2]]){
            s.referrer_section = s.getSectionInfo(s.aa_pref, a_referrer_url[2], a_referrer_url[3]);
        }

        // --- Define contents name(e.g. pageName) --------------------------
        // define pageType,PageName,prop1,prop2,prop3,prop7

        // [pageName] pageName
        // ---------------------------------------------------------
        if (("pageType" in s.aa_data.status && s.aa_data.status.pageType === "errorPage") || s.pageType === "errorPage") {
            s.pageType = "errorPage";
            s.pageName = "404:" + s.wd.location.protocol + "//" + s.wd.location.hostname + s.wd.location.pathname;
        } else {
            if (s.aa_data.status.pageName) {
                s.pageName = s.siteID + ":" + s.aa_data.status.pageName;
            } else {
                s.pageName = s.getPageName(s.getLocation.href());

                var coupon_url = s.getLocation.href().indexOf("shopping.yahoo.co.jp/coupon");
                var products_url = s.getLocation.href().indexOf("shopping.yahoo.co.jp/products");
                if (coupon_url !== -1) {
                    if (s.aa_data.status.device != 3) {
                        s.pageName = s.pageName.replace(/^shp:coupon:coupon/, "shp:coupon");
                    } else {
                        s.pageName = s.pageName.replace(/^shp:sp_coupon:coupon/, "shp:sp_coupon");
                    }
                }
                else if (products_url !== -1) {
                    if (s.aa_data.status.device != 3) {
                        s.pageName = s.pageName.replace(/^shp:main:products/, "shp:main:product");
                    } else {
                        s.pageName = s.pageName.replace(/^shp:sp_main:products/, "shp:sp_main:product");
                    }
                }
                else if (s.getLocation.hostname() == "talk.shopping.yahoo.co.jp") {
                    if (s.getLocation.pathname().indexOf("/contact/") == 0) {
                        if (s.aa_data.status.device != 3) {
                            s.pageName = s.pageName.replace(/^shp:store:contact:([^:]+).*/, "shp:store:store:$1:talk:contact")
                        } else {
                            s.pageName = s.pageName.replace(/^shp:sp_store:contact:([^:]+).*/, "shp:sp_store:sp_store:$1:talk:contact")
                        }
                    }
                    else if (s.getLocation.pathname().indexOf("/list/") == 0) {
                        if (s.aa_data.status.device != 3) {
                            s.pageName = s.pageName.replace(/^shp:talk:list:.*/, "shp:talk:list:index")
                        } else {
                            s.pageName = s.pageName.replace(/^shp:sp_talk:list:.*/, "shp:sp_talk:list:index")
                        }
                    }
                    else if (s.getLocation.pathname().indexOf("/detail/") == 0) {
                        if ("talk_store_id" in s.aa_data.status && s.aa_data.status.talk_store_id) {
                            if (s.aa_data.status.device != 3) {
                                s.pageName = s.pageName.replace(/^shp:store:detail:.*/, "shp:store:store:" + s.aa_data.status.talk_store_id + ":talk:detail")
                            } else {
                                s.pageName = s.pageName.replace(/^shp:sp_store:detail:.*/, "shp:sp_store:sp_store:" + s.aa_data.status.talk_store_id + ":talk:detail")
                            }
                        }
                    }
                }
            }
        }
        if ("pageInfo" in s.aa_data.status) {
            if (s.aa_data.status.pageInfo === "item") {
                s.pageName = s.roundPageName(s.pageName, "detail");
            } else if (s.aa_data.status.pageInfo === "category") {
                s.pageName = s.roundPageName(s.pageName, "category");
            } else if (s.aa_data.status.pageInfo === "custom") {
                s.pageName = s.roundPageName(s.pageName, "custom");
            } else if (s.aa_data.status.pageInfo === "other") {
                s.pageName = s.roundPageName(s.pageName, "other");
            } else if (s.aa_data.status.pageInfo) {
                s.pageName += ":" + s.aa_data.status.pageInfo;
            }
        }
        //[subsection_suffix]
        var subsection_suffix= null;
        if ("subsection_suffix" in s.aa_data.status && s.aa_data.status.subsection_suffix.length >= 1) {
            subsection_suffix = s.aa_data.status.subsection_suffix;
        }
        // [event] import event by pagecode
        // ---------------------------------------------------------
        if ("events" in s.aa_data.status) {
            s.events = s.apl(s.events, s.aa_data.status.events, ",", 1);
        }
        s.raw_sc_i = s.getQueryParam("sc_i");

        // [prop1] distinction of Subdomain
        // ---------------------------------------------------------
        s.prop1 = s.siteID;

        // [prop2] Section
        // [prop3] Sub-Section
        // [prop8] Store function Sub-Section
        // [prop9] Store function SubSub-Section
        // ---------------------------------------------------------
        // domain: s.getLocation.hostname
        // path: s.getLocation.pathname
        // domain+path: current_domainandpath
        var current_domainandpath = s.getLocation.hostname() + s.getLocation.pathname();

        s.section_object = s.getSectionInfo(s.aa_pref);
        //[subsection_suffix] for search
        if(!s.eo && !s.lnk && s.section_object.section === "search" && subsection_suffix){
            s.pageName += ":" + subsection_suffix;
            s.section_object.subsection      = s.section_object.subsection      + ":" + subsection_suffix;
            s.section_object.only_subsection = s.section_object.only_subsection + ":" + subsection_suffix;
            s.c_w("search_suffix", subsection_suffix);
        }

        s.prop2 = s.generateSectionNameString(s.section_object.section, s.sectionname_prefix, s.siteID);
        s.prop3 = s.generateSectionNameString(s.section_object.subsection, s.sectionname_prefix, s.siteID);


        // "store" section exception handling
        if(s.section_object.section === "store"){

            if(s.section_object.path_arr.length === 1){
                // prop8 is Store axis subsection.
                s.prop8 = s.generateSectionNameString("store" + ":" + "top", s.sectionname_prefix, s.siteID);
                s.prop9 = s.prop3 + ":" + "top";
                s.store_axis_only_subsection = "top";
            }else if(s.section_object.path_arr.length > 1){
                if("pageInfo" in s.aa_data.status && s.aa_data.status.pageInfo){
                    // prop8 is Store axis subsection.
                    s.prop8 = s.generateSectionNameString(s.section_object.section + ":" + s.aa_data.status.pageInfo, s.sectionname_prefix, s.siteID);
                    s.prop9 = s.prop3 + ":" + s.aa_data.status.pageInfo;
                    s.store_axis_only_subsection = s.aa_data.status.pageInfo;
                }else{
                    // prop8 is Store axis subsection.
                    s.prop8 = s.generateSectionNameString(s.section_object.section + ":" + s.section_object.path_arr[1], s.sectionname_prefix, s.siteID);
                    s.prop9 = s.prop3 + ":" + s.section_object.path_arr[1];
                    s.store_axis_only_subsection = s.section_object.path_arr[1];
                }
            }

        // "storestampcard" section exception handling
        }else if(s.section_object.section === "my" && s.section_object.only_subsection === "storestampcard"){

            var stampcard_suffix = "detail";
            if (s.section_object.path_arr.length === 2){
                stampcard_suffix = "list";
            }
            s.prop8 = s.generateSectionNameString(s.section_object.subsection + ":" + stampcard_suffix, s.sectionname_prefix, s.siteID);

        }else{
            s.prop8 = s.generateSectionNameString(s.section_object.subsection, s.sectionname_prefix, s.siteID);
            s.prop9 = "D=c3";
            s.store_axis_only_subsection = s.section_object.only_subsection;
        }


        // [campaign,eVar7-eVar10] External Campaign Code
        // ---------------------------------------------------------
        s.campaign = s.campaign || s.getQueryParam("sc_e", "", s.getLocation.href());
        s.eVar7 = s.eVar8 = s.eVar9 = s.eVar10 = "D=v0";

        // [prop7/eVar42] Category No.
        // ---------------------------------------------------------
        if("categoryID" in s.aa_data.status){
            s.prop7 = s.aa_data.status.categoryID;
        }
        else if (s.section_object.section === "category" || s.section_object.section === "ranking") {

            var matching_category_arr = [];
            var matching_regex = null;

            if(s.section_object.section === "category" && s.section_object.only_subsection === "list"){

                matching_regex = /^(?:category\.shopping\.yahoo\.co\.jp\/list\/([0-9]+)|shopping\.yahoo\.co\.jp\/category\/((?:[0-9]+\/)+))/;

            }else if(s.section_object.section === "category" && s.section_object.only_subsection === "recommend"){

                matching_regex = /^shopping\.yahoo\.co\.jp\/category\/((?:[0-9]+\/)+)recommend/;

            }else if(s.section_object.section === "ranking" && s.section_object.only_subsection === "list"){

                matching_regex = /^shopping\.yahoo\.co\.jp\/category\/((?:[0-9]+\/)+)ranking/;

            }else if(s.section_object.section === "ranking"){

                matching_regex = /^shopping\.yahoo\.co\.jp\/(?:ranking\/(?:(?:.+\/)+))/;

            }

            if(matching_regex){

                matching_category_arr = current_domainandpath.match(matching_regex);

                if(matching_category_arr){
                    for(var i_cate=1,len=matching_category_arr.length;i_cate<len;i_cate++){
                        if(matching_category_arr[i_cate]){
                            if(matching_category_arr[i_cate].slice(-1) === "/"){
                                s.prop7 = matching_category_arr[i_cate].slice(0,-1);
                            }else{
                                s.prop7 = matching_category_arr[i_cate];
                            }
                        }
                    }
                }
            }
        }

        if(s.prop7){
            s.eVar42 = "D=c7";
        }

        // [prop10] Display device type
        // ---------------------------------------------------------
        if("device" in s.aa_data.status){
            s.prop10 = s.aa_data.status.device;
        }else{
            // default value is "1"(PC)
            s.prop10 = 1;
        }

        // [eVar4] business
        // ---------------------------------------------------------
        if ("account_type" in s.aa_data.status && s.aa_data.status.account_type.length > 0) {
            if (s.aa_data.status.account_type == "business_id") {
                s.eVar4 = "business";
            }
        }

        // [eVar5] New/Repeat/RepeatLite
        // ---------------------------------------------------------
        if ("visitor_type_per_month" in s.aa_data.status && s.aa_data.status.visitor_type_per_month.length > 0) {
            s.eVar5 = s.aa_data.status.visitor_type_per_month;
        }else{
            s.eVar5 = "New";
        }

        // [prop11,eVar11] New/Repeat
        // ---------------------------------------------------------
        if ("visitor_type" in s.aa_data.status && s.aa_data.status.visitor_type.length > 0) {
            s.prop11 = s.aa_data.status.visitor_type;
        }else{
            s.prop11 = "New";
        }
        s.eVar11 = "D=c11";

        // [prop12,eVar12] Time Parting
        // ---------------------------------------------------------
        s.prop12 = s.getTimeParting("h", "9", "", "0") + "-" + s.getTimeParting("d", "9", "", "0");
        s.eVar12 = "D=c12";

        // [eVar14] Promotion(Campaign) Name
        // ---------------------------------------------------------
        // check promotion_id
        if ("promotion_id" in s.aa_data.status){
            if(s.aa_data.status.promotion_id === "no_promo"){
                s.eVar14 = "";
            } else {
                // change section/subsection value
                s.section_object = {
                    section: "campaign",
                    subsection: "campaign",
                    only_subsection: ""
                };
                s.prop2 = s.generateSectionNameString(s.section_object.section, s.sectionname_prefix, s.siteID);
                s.prop3 = s.generateSectionNameString(s.section_object.subsection, s.sectionname_prefix, s.siteID);
                s.prop8 = s.prop3;
                s.prop9 = s.prop3;

                s.eVar14 = s.aa_data.status.promotion_id;
                if(s.aa_data.status.promotion_id === null || s.aa_data.status.promotion_id === ""){
                    s.eVar14 = "unknown";
                }
            }

        } else if (s.section_object.section === "campaign") {
            var a_matching_hostname = s.getLocation.hostname().match(/^(.*?)\.yahoo\.co\.jp$/);
            var a_matching_pathname = s.getLocation.pathname().split("/").slice(1);

            // not "shopping.yahoo.co.jp"
            if(!a_matching_hostname){
                s.eVar14 = s.getLocation.hostname();
            // type1
            }else if (a_matching_pathname.length >= 1) {
                var sp_auto = (
                        a_matching_hostname[1] === "topics.shopping" &&
                        a_matching_pathname[0] === "m" &&
                        a_matching_pathname.length >= 2 &&
                        a_matching_pathname[1].match(/^(fair|category)$/)
                        ) || (
                        a_matching_hostname[1] === "topics.shopping" &&
                        a_matching_pathname.length >= 1 &&
                        a_matching_pathname[0].match(/^fair$/));
                s.eVar14 = a_matching_hostname[1];
                for (var i = 0, len = a_matching_pathname.length; i < len; i++) {
                    if (a_matching_pathname[i] !== ""){
                        var index_to_html = a_matching_pathname[i].indexOf(".html");
                        if(index_to_html == -1){
                            s.eVar14 += ":" + a_matching_pathname[i];
                        }else if(sp_auto){
                            var last_word_index_check = a_matching_pathname[i].substring(0, index_to_html);
                            if (last_word_index_check !== "index") {
                                s.eVar14 += ":" + last_word_index_check;
                            }
                        }
                    }
                    if ((a_matching_hostname[1] === "contents.shopping" || a_matching_hostname[1] === "shopping") && i >= 1) {
                        break;
                    }
                }
            }
        }
        // [prop17] store category level
        // ---------------------------------------------------------
        if ("store_category_level" in s.aa_data.status) {
            s.prop17 = s.aa_data.status.store_category_level;
        }
        // [eVar26/event33] MT search
        // ---------------------------------------------------------
        if(s.eVar14 && s.eVar14 !== ""){
            var is_mts = false;
            var mts_params = ["keyword", "uIv", "uSa", "delivery", "ship", "ship2", "ndd", "ea", "pr_min", "pf", "pr_max", "pt", "cid", "brandid", "spec"];
            var mts_conditions = [s.eVar14.replace(/^shopping:/, "")];
            for(i = 0;i < mts_params.length;i++){
                var mts_value = s.getQueryParam(mts_params[i]);
                if(mts_value && mts_value !== ""){
                    is_mts = true;
                    mts_conditions.push(mts_params[i] + "=" + mts_value);
                }
            }
            if(is_mts === true){
                s.eVar26 = mts_conditions.join("/");
                s.events = s.apl(s.events,"event33", ",", 1);
            }
        }

        // [prop14/eVar23] brandID
        // ---------------------------------------------------------
        if("brandID" in s.aa_data.status){
            s.prop14 = s.aa_data.status.brandID;
            s.eVar23 = "D=c14";
        }

        // [eVar15/eVar28/eVar36] Internal Campaign Code
        // [eVar40] Referrer domain since last touch
        // ---------------------------------------------------------
        if (s.getReferrer()) {
            var s_referrer_domain = "";
            var a_internal_domain = s.linkInternalFilters.split(",");
            var i_internal_matching = 0;

            s_referrer_domain = s.wd.document.referrer.match(/^[httpsfile]+:\/{2,3}([0-9a-z\.\-:]+.*?):?[0-9]*?\/{0,1}/i)[1];
            a_internal_domain = s.linkInternalFilters.split(",");
            for (var k = 0, l = a_internal_domain.length; k < l; k++) {
                if (s_referrer_domain.indexOf(a_internal_domain[k]) > -1) {
                    i_internal_matching = 1;
                    break;
                }
            }
            s._intmatchflg = i_internal_matching;
            if (i_internal_matching === 0) {
                s.eVar40 = s_referrer_domain;
            }
            var order2order = s.isSubsection([["order"]], s.subDomID, s.section_object.section, s.section_object.only_subsection) &&
                              s_referrer_domain === "order.shopping.yahoo.co.jp";
            if (!order2order && i_internal_matching === 1 && !s.eVar15){
                s.eVar15 = s.raw_sc_i;
                s.eVar28 = "D=v15";
                s.eVar36 = "D=v15";
            }
        }
        else if (app_type) {
            s.eVar15 = s.raw_sc_i;
            s.eVar28 = "D=v15";
            s.eVar36 = "D=v15";
        }

        // [eVar95] Cart Campaign Code
        s.eVar95 = s.getQueryParam("sc_i_c");

        // [eVar16,eVar17] Finding Method
        // ---------------------------------------------------------
        if (s.aa_data.status.pageInfo === "item") {
            if(!s.getReferrer()) {
                s.eVar16 = "direct";
                s.eVar17 = "D=v16";
            }else if(!a_referrer_url) {
                s.eVar16 = "unknown(parse error)";
                s.eVar17 = "D=v16";
            }else if(s.referrer_section && s.referrer_section.section){
                s.eVar16 = s.channel + ":" + s.subDomLst[a_referrer_url[2]] + ":" + s.referrer_section.subsection;
                if(!s.eo && !s.lnk && s.referrer_section.section === "search" && s.c_r("search_suffix")){
                    s.eVar16 += ":" + s.c_r("search_suffix");
                }
                s.eVar17 = "D=v16";
            }else{
                s.eVar16 = "unknown(external site)";
                s.eVar17 = "D=v16";
            }
        }

        // [eVar19] Payment Type
        // ---------------------------------------------------------
        if ("payment_type" in s.aa_data.status) {
            s.eVar19 = s.aa_data.status.payment_type;
        }

        // [eVar20] Shipment Type
        // ---------------------------------------------------------
        if ("shipment_type" in s.aa_data.status) {
            s.eVar20 = s.aa_data.status.shipment_type;
        }

        // [eVar21] Search condition of Category Name
        // ---------------------------------------------------------
        if ("search_category_name" in s.aa_data.status) {
            s.eVar21 = s.aa_data.status.search_category_name;
        }

        // [eVar13] Search Keywords
        // [event2] keyword search count(adult)
        // [event11] keyword search count
        //
        // "eVar13" measure search keywords. Search keywords track not only "search.shopping.yahoo.co.jp",but also "campaign page".
        // (event11 is only track item search in "search.shopping.yahoo.co.jp")
        // (event2 is only track item search in "earch.adult.shopping.yahoo.co.jp")
        //
        //
        // If URL matches below conditions, not measure search keywords and search counts.
        //   - referrer is not "search.shopping.yahoo.co.jp" or "shopping.search.yahoo.co.jp"
        //  AND
        //   - set "external campaign code"
        //   - OR: set "internal campaign code" and it not includes "searchBox"
        //   - OR: set "rkf" query parameter equal "1"
        // ---------------------------------------------------------
        if (s.section_object.subsection.indexOf("search") === 0) {
            var v13_flag = false;
            var rkf = s.getQueryParam("rkf");
            //if ((s.wd.document.referrer.indexOf("search.shopping.yahoo.co.jp") == -1 && s.wd.document.referrer.indexOf("shopping.search.yahoo.co.jp") == -1) && (s.campaign || s.eVar15.indexOf("searchBox") == -1 || s.getQueryParam("rkf") == "1")) {
            if ((s.wd.document.referrer.indexOf("search.adult.shopping.yahoo.co.jp") >= 0) ||
                (!s.campaign && s.raw_sc_i.indexOf("searchBoxAdult") >= 0 && rkf !== "1")) {
                v13_flag = true;
                // countup keyword search
                s.events = s.apl(s.events, "event2", ",", 1);
            }else if ((s.wd.document.referrer.indexOf("search.shopping.yahoo.co.jp") >= 0 || s.wd.document.referrer.indexOf("shopping.search.yahoo.co.jp") >= 0) ||
                (!s.campaign && s.raw_sc_i.indexOf("searchBox") >= 0 && rkf !== "1")) {
                v13_flag = true;
                // countup keyword search
                s.events = s.apl(s.events, "event11", ",", 1);
            }
            if(v13_flag){
                if("search_keyword" in s.aa_data.status){
                    s.eVar13 = s.aa_data.status.search_keyword;
                }
                s.eVar13 = s.eVar13.replace(/\u3000/g, " ");
                s.eVar13 = s.eVar13.replace(/[\uff21-\uff3a\uff41-\uff5a\uff10-\uff19]/g, function(str) {
                    return String.fromCharCode(str.charCodeAt(0) - 65248);
                });
                s.eVar13 = s.convHtoF(s.eVar13);
                s.eVar13 = s.eVar13.toLowerCase();
            } else {
                s.eVar13 = s.eVar21 = "";
                s.events = s.events.replace(/,event3/, "");
                s.events = s.events.replace(/,event12/, "");
            }
        }else if(s.section_object.section === "campaign"){
            // if campaign site, get query param "p";
            try{
                s.eVar13 = s.getQueryParam("p");
            }catch(e){}
        }

        // [prop27/event33] get Load time
        // [eVar37] pageName
        // ---------------------------------------------------------
        s.prop27 = s.getLoadTime();
        if(s.prop27){
            s.events=s.apl(s.events,'event35='+s.prop27,',',1);
        }
        if(s.pageName){
            s.eVar37 = "D=pageName";
        }

        // [eVar29] First Access Date
        // ---------------------------------------------------------
        var d_current_date = new Date();
        s.eVar29 = d_current_date.getFullYear() + "-" + (d_current_date.getMonth() + 1) + "-" + d_current_date.getDate();

        // [eVar31] Internal Tracking Code for sales promotion
        // ---------------------------------------------------------
        if (s.eVar15) {
            if (s.eVar15.indexOf("_promo_") > -1) {
                s.eVar31 = "D=v15";
            }
        }

        // [eVar33] Product in stock(by cart detail page)
        // ---------------------------------------------------------
        if ("cart_inStock" in s.aa_data.status) {
            if (s.aa_data.status.cart_inStock === true) {
                s.eVar33 = "in stock";
            } else if (s.aa_data.status.cart_inStock === false) {
                s.eVar33 = "out stock";
            }
        }

        // [eVar34/prop34] login status
        // ---------------------------------------------------------
        if ("login" in s.aa_data.status && s.aa_data.status.login === true) {
            s.eVar34 = "Login";
        } else {
            s.eVar34 = "NoLogin";
        }
        s.prop34 = "D=v34";

        // [eVar38/prop28/event34] Page scroll information
        // eVar38: Previous pageName
        // prop28: Page scroll information
        // event34: Page scroll(%)
        // ---------------------------------------------------------
        if(!s.eo && !s.lnk){
            var ppv = s.getPercentPageViewed(s.pageName);
            var ppv_pn = s.getPreviousValue(s.pageName,'s_ppn');
            //If an array was returned, there was a previous page view - i.e. data was captured
            if(ppv && typeof ppv === 'object' && typeof ppv.length === 'number' && ppv.length > 2 && ppv[0] === ppv_pn) {
                s.prop28 = ppv;
                // eVar38: prev pageName
                s.eVar38 = ppv[0];
                // event34 prev page scroll %(max)
                s.events = s.apl(s.events,"event34="+ppv[2], ",", 1);
            }
        }

        // [eVar41] Yahoo! Member Demographic
        // null: Visitor
        // ---------------------------------------------------------
        if ("login" in s.aa_data.status && s.aa_data.status.login === true) {
            var a_demographic = [];
            if ("demographic" in s.aa_data && "age" in s.aa_data.demographic && s.aa_data.demographic.age) {
                a_demographic.push(s.aa_data.demographic.age);
            }else{
                a_demographic.push("n");
            }
            if ("demographic" in s.aa_data && "gender" in s.aa_data.demographic && s.aa_data.demographic.gender) {
                a_demographic.push(s.aa_data.demographic.gender);
            }else{
                a_demographic.push("n");
            }
            if ("demographic" in s.aa_data && "premium" in s.aa_data.demographic && s.aa_data.demographic.premium) {
                a_demographic.push(s.aa_data.demographic.premium);
            }else{
                a_demographic.push("n");
            }
            if ("demographic" in s.aa_data && "starclub_rank" in s.aa_data.demographic && s.aa_data.demographic.starclub_rank) {
                a_demographic.push(s.aa_data.demographic.starclub_rank);
            }else{
                a_demographic.push("n");
            }
            s.eVar41 = a_demographic.join(":");
        } else {
            s.eVar41 = "g:g:g:g";
        }

        // for yjcard
        var yjcard_status = "g";
        if ("login" in s.aa_data.status && s.aa_data.status.login === true) {
            yjcard_status = "n";
            if ("demographic" in s.aa_data && "yjcard" in s.aa_data.demographic && s.aa_data.demographic.yjcard !== "") {
                yjcard_status = s.aa_data.demographic.yjcard + '';
            }
        }
        s.eVar60 = yjcard_status;

        // [prop42] campaign module implression
        // ---------------------------------------------------------
        if ("exception" in s.aa_data && "campaign_mod_imps" in s.aa_data.exception) {
            s.prop42 = s.aa_data.exception.campaign_mod_imps;
        }

        // [prop43] display module name
        // ---------------------------------------------------------
        if("display_module" in s.aa_data.status){
            s.prop43 = s.aa_data.status.display_module;
        }
        // [eVar43] category_standard_id
        // ---------------------------------------------------------
        if("category_standard_id" in s.aa_data.status){
            s.eVar43 = s.aa_data.status.category_standard_id;
        }

        // [eVar44] click breadcrumb Information
        // ---------------------------------------------------------
        if(!s.eo && !s.lnk){
            if(s.c_r("breadcrumb")){
                s.eVar44 = s.c_r("breadcrumb");
                s.c_w("breadcrumb","");
            }
        }

        // [eVar45] store search
        // ---------------------------------------------------------
        if(s.section_object.section === "store" && s.store_axis_only_subsection && s.store_axis_only_subsection === "search"){
            s.eVar45 = s.storeSearchParams();
            // take p value for eVar46 only when sp(it fails for pc since it is encoded in EUC-JP)
            if (s.sectionname_prefix === "sp") {
                var keyword = s.getQueryParam("p");
                if(keyword){
                    s.eVar46 = keyword;
                }
            }
        }
        // [eVar47] espec of category-list from parameter
        // ---------------------------------------------------------
        if((s.section_object.section === "category" && s.section_object.only_subsection === "list") ||
           (s.section_object.section === "search"   && s.section_object.only_subsection.indexOf("search") === 0)){
            var espec = s.getQueryParam("espec");
            if(espec){
                s.eVar47 = espec;
            }
        }
        // [eVar48] location from where the vistor flows into the search area
        // ---------------------------------------------------------
        if(s.isSubsection([["search","search"]], s.subDomID, s.section_object.section, s.section_object.only_subsection) && !s.eo && !s.lnk){
            //currently in search area
            raw_sc_e = s.getQueryParam("sc_e");
            if(raw_sc_e && raw_sc_e.length > 0){
                //parameter sc_e exists
                s.eVar48 = raw_sc_e;
            }else if (!s.getReferrer()) {
                //no referrer
                s.eVar48 = "direct";
            }else if (!a_referrer_url || a_referrer_url.length < 2){
                //illegal referrer
                s.eVar48 = "unknown";
            }else if(s.subDomLst[a_referrer_url[2]]){
                //inside shopping
                if (s.referrer_section && s.referrer_section.section) {
                    // if referrer section is out of search area, measure it.
                    if(!s.isSubsection([["search","search"]], s.subDomLst[a_referrer_url[2]], s.referrer_section.section, s.referrer_section.only_subsection)){
                        s.eVar48 = "section:" + s.channel + ":" + s.subDomLst[a_referrer_url[2]] + ":" + s.referrer_section.subsection;
                    }
                    //no else phrase means no measure for any transition from search area to search area
                } else {
                    //failed to get section
                    s.eVar48 = "unknown";
                }
            }else if(a_referrer_url[2].match(/^(search\.yahoo\.co\.jp|www\.google\.(com|co\.jp))$/)){
                //domain is either "search.yahoo.co.jp", "www.google.com" or "www.google.co.jp"
                s.eVar48 = "search:" + a_referrer_url[2];
            }else if(a_referrer_url[2].match(/.*\.yahoo\.co\.jp$/)){
                //one of other yahoo service
                s.eVar48 = "otherYahoo:" + a_referrer_url[2];
            }else{
                //anything else
                s.eVar48 = "referrer:External domain";
            }
        }
        // [eVar49] Site Transition Flow
        // ---------------------------------------------------------
        if(!s.eo && !s.lnk){
            s.eVar49 = s.manageHistory("eVar49", 5, ">", s.prop8);
        }
        // [eVar51] Search Sort Type
        // ---------------------------------------------------------
        if ("search_sort_type" in s.aa_data.status) {
            s.eVar51 = s.aa_data.status.search_sort_type;
        }
        // [eVar52] Search Condition
        // ---------------------------------------------------------
        if ("search_condition" in s.aa_data.status) {
            s.eVar52 = s.aa_data.status.search_condition;
        }
        // [eVar53] Search Brand
        // ---------------------------------------------------------
        if ("search_brand" in s.aa_data.status) {
            s.eVar53 = s.aa_data.status.search_brand;
        }

        // [eVar55] Purchase Type
        // ---------------------------------------------------------
        if ("purchase_type" in s.aa_data.status) {
            s.eVar55 = s.aa_data.status.purchase_type;
        }

        // [eVar56] Store&Review Transition Flow
        // ---------------------------------------------------------
        if(s.prop8 && !s.eo && !s.lnk){
            // definition for what store area is
            var store_area_settings = [["store","store"], ["store.adult","store"]];
            var latest_value;
            if(s.isSubsection(store_area_settings, s.subDomID, s.section_object.section, s.section_object.only_subsection)){
                //in store area
                //change store?
                var change_store = false;
                if(s.referrer_section && s.referrer_section.section) {

                    // from item page in different store
                    if(s.isSubsection(store_area_settings, s.subDomLst[a_referrer_url[2]], s.referrer_section.section, s.referrer_section.only_subsection)) {
                        change_store = s.section_object.only_subsection !== s.referrer_section.only_subsection;

                    // from store review page in different store
                    } else if(s.isSubsection([["review","store"]], s.subDomLst[a_referrer_url[2]], s.referrer_section.section, s.referrer_section.only_subsection)) {
                        change_store = s.section_object.only_subsection !== s.d.referrer.split("/")[4];

                    // from item review page in different store
                    } else if(s.isSubsection([["main","review"]], s.subDomLst[a_referrer_url[2]], s.referrer_section.section, s.referrer_section.only_subsection)) {
                        change_store = s.section_object.only_subsection !== s.getQueryParam("store_id", "", s.d.referrer);
                    }
                }
                //create latest value;
                index_colon = s.prop8.lastIndexOf(":");
                latest_value = (change_store?"CS:":"") + (index_colon < 0?s.prop8:s.prop8.substring(index_colon + 1));
            }else if(s.isSubsection([["review","store"]], s.subDomID, s.section_object.section, s.section_object.only_subsection)){
                //in store review
                latest_value = "REV_STORE";
            }else if(s.isSubsection([["main","review"]], s.subDomID, s.section_object.section, s.section_object.only_subsection)){
                //in item review
                latest_value = "REV_ITEM";
            }else if(s.isSubsection([["order"]], s.subDomID, s.section_object.section, s.section_object.only_subsection)){
                //in cart area
                latest_value = "CART";
            }else{
                //anything else
                latest_value = "OTHER";
            }
            //manage history
            s.eVar56 = s.manageHistory("eVar56", 10, ">", latest_value);
        }

        // [eVar61] Top: view pattern(filter funciton)
        // ---------------------------------------------------------
        if ("top_design_pattern" in s.aa_data.status) {
            s.eVar61 = s.aa_data.status.top_design_pattern;
        }

        // [eVar62] CategoryRecommend: view pattern(filter funciton)
        // ---------------------------------------------------------
        if ("catereco_design_pattern" in s.aa_data.status) {
            s.eVar62 = s.aa_data.status.catereco_design_pattern;
        }

        // [eVar63] Store: view pattern(filter funciton)
        // ---------------------------------------------------------
        if ("store_design_pattern" in s.aa_data.status) {
            s.eVar63 = s.aa_data.status.store_design_pattern;
        }

        // [eVar64] Cart: view pattern(filter funciton)
        // ---------------------------------------------------------
        if ("cart_design_pattern" in s.aa_data.status) {
            s.eVar64 = s.aa_data.status.cart_design_pattern;
        }

        // [eVar65] MT(Campaign): view pattern(filter funciton)
        // ---------------------------------------------------------
        if ("mt_design_pattern" in s.aa_data.status) {
            s.eVar65 = s.aa_data.status.mt_design_pattern;
        }

        // [eVar68] Backend page: view pattern(filter funciton)
        // ---------------------------------------------------------
        if ("backend_design_pattern" in s.aa_data.status) {
            s.eVar68 = s.aa_data.status.backend_design_pattern;
        }

        // [eVar90] Backend QCS: view pattern(filter funciton)
        // ---------------------------------------------------------
        if ("beqcs_design_pattern" in s.aa_data.status) {
            s.eVar90 = s.aa_data.status.beqcs_design_pattern;
        }

        // [eVar89] Recommend page: view pattern(filter funciton)
        // ---------------------------------------------------------
        if ("recommend_design_pattern" in s.aa_data.status) {
            s.eVar89 = s.aa_data.status.recommend_design_pattern;
        }

        // [eVar69] Search/CategoryPage: view pattern(filter funciton)
        // ---------------------------------------------------------
        if ("search_design_pattern" in s.aa_data.status) {
            s.eVar69 = s.aa_data.status.search_design_pattern;
        }

        // [eVar85] Product(kainavi): view pattern(filter funciton)
        if ("product_design_pattern" in s.aa_data.status) {
            s.eVar85 = s.aa_data.status.product_design_pattern;
        }

        // [eVar86] MT2: view pattern(filter funciton)
        // ---------------------------------------------------------
        if ("mt2_design_pattern" in s.aa_data.status) {
            s.eVar86 = s.aa_data.status.mt2_design_pattern;
        }

        // [eVar87] Ranking: view pattern(filter funciton)
        if ("ranking_design_pattern" in s.aa_data.status) {
            s.eVar87 = s.aa_data.status.ranking_design_pattern;
        }
        
        // [eVar88] Cart2: view pattern(filter funciton)
        // ---------------------------------------------------------
        if ("cart2_design_pattern" in s.aa_data.status) {
            s.eVar88 = s.aa_data.status.cart2_design_pattern;
        }

        // [eVar70] Search/CategoryPage: view pattern(related word)
        // ---------------------------------------------------------
        if ("search_related_pattern" in s.aa_data.status) {
            s.eVar70 = s.aa_data.status.search_related_pattern;
        }
        if ("search_dym_pattern" in s.aa_data.status) {
            s.eVar70 = s.aa_data.status.search_dym_pattern;
        }


        // [eVar71] search related keyword click area
        // [eVar71] click Did you mean area in search page
        // ---------------------------------------------------------
        if (!s.eo && !s.lnk){
            if(s.c_r("search_related_pos")){

                s.eVar71 = s.c_r("search_related_pos");
                // reset value
                s.c_w("search_related_pos","",0);
            }
            if(s.c_r("search_dym_link")){

                s.eVar71 = "search_speller";
                // reset value
                s.c_w("search_dym_link","",0);
            }
        }

        // [eVar72] search result click area
        // [event32] search result click count
        // ---------------------------------------------------------
        if (!s.eo && !s.lnk){
            if(s.c_r("search_result_pos")){

                s.eVar72 = s.c_r("search_result_pos");
                s.events = s.apl(s.events,"event32", ",", 1);

                // reset value
                s.c_w("search_result_pos","",0);
            }

        }


        // [eVar73] Section that was allowed to flow into the store area
        // [eVar74] Subsection that was allowed to flow into the store area
        // ---------------------------------------------------------
        if(s.isSubsectionStore(s.subDomID, s.section_object.section, s.section_object.only_subsection) && !s.eo && !s.lnk){
            // exist document.referrer
            if (!s.getReferrer()) {
                s.eVar73 = "direct(or HTTPS)";
                s.eVar74 = "D=v73";
            }else if (!a_referrer_url) {
                s.eVar73 = "unknown(parse error)";
                s.eVar74 = "D=v73";
            }else if(a_referrer_url.length < 2){
                s.eVar73 = "unknown(illegal referrer url format)";
                s.eVar74 = "D=v73";
            }else if(s.subDomLst[a_referrer_url[2]]){
                if (s.referrer_section && s.referrer_section.section) {
                    // if section equal is out of either store-area, cart, item_opt(put into cart from favorite) or inform, don't measure it.
                    if(!s.isSubsectionStore(s.subDomLst[a_referrer_url[2]], s.referrer_section.section, s.referrer_section.only_subsection) &&
                        s.referrer_section.section !== "order" &&
                        s.referrer_section.section !== "item_opt" &&
                        s.referrer_section.section !== "inform"){
                        s.eVar73 = s.channel + ":" + s.subDomLst[a_referrer_url[2]] + ":" + s.referrer_section.section;
                        s.eVar74 = s.channel + ":" + s.subDomLst[a_referrer_url[2]] + ":" + s.referrer_section.subsection;
                        if(s.referrer_section.section === "search" && s.c_r("search_suffix")){
                            s.eVar74 += ":" + s.c_r("search_suffix");
                        }
                    }
                } else {
                    s.eVar73 = "unknown(can't identify section)";
                    s.eVar74 = "D=v73";
                }
            }else if(a_referrer_url[2].match(/.*\.yahoo\.co\.jp$/)){
                s.eVar73 = "yahooOtherService(" + a_referrer_url[2] + ")";
                s.eVar74 = "D=v73";
            }else{
                s.eVar73 = "external site";
                s.eVar74 = "D=v73";
            }
        }


        // [prop74] debugging url
        // ---------------------------------------------------------
        if (s.section_object.section === "other") {
            s.prop74 = "D=g";
        }


        // [prop75] s_code version
        // ---------------------------------------------------------
        s.prop75 = s.code_version;


        //[eVar77]  carrier info
        // ---------------------------------------------------------
        if ("demographic" in s.aa_data && "carrier_info" in s.aa_data.demographic) {
            var target = s.aa_data.demographic.carrier_info ? s.aa_data.demographic.carrier_info.split(":") : null;
            
            if (target && target.length == 2 && target[0] && target[1]) {
                s.eVar77 = s.aa_data.demographic.carrier_info;
            }
        }


        //[eVar78]  Store advertising tracking code
        //[eVar79]  Store advertising tracking code
        //[eVar80]  Store advertising tracking code
        // ---------------------------------------------------------
        if (s.campaign || s.eVar15) {
            var target = null;
            
            if (s.campaign) {
                if (s.campaign.match(/mcad$/)) {
                    target = s.campaign;
                }
            } else {
                if (s.eVar15.match(/mcad$/) || s.eVar15.match(/_k2$/) || s.eVar15.match(/_mdSELBN$/)) {
                    target = s.eVar15;
                } else if (s.eVar15.match(/_mdRECOMMEND_[0-9]+$/)) {
                    target = s.eVar15.replace(/_mdRECOMMEND_[0-9]+$/, "_mdRECOMMEND");
                }
            }
            
            if(target){
               s.eVar78 = target;
            }            
        }
        
        if (s.eVar72) {
            var target = null;
            
            if (s.eVar72.indexOf("item=catelist|type=ad") > -1) {
                if (s.aa_data.status.device != 3) {
                    target = "shp_pc_cate-list_im";
                } else {
                    target = "shp_sp_cate-list_im";
                }
            } else if (s.eVar72.indexOf("item=catelist|type=k2") > -1) {
                if (s.aa_data.status.device != 3) {
                    target = "shp_pc_cate-list_k2";
                } else {
                    target = "shp_sp_cate-list_k2";
                }
            } else if (s.eVar72.indexOf("item=search|type=ad") > -1) {
                if (s.aa_data.status.device != 3) {
                    target = "shp_pc_search_im";
                } else {
                    target = "shp_sp_search_im";
                }
            } else if (s.eVar72.indexOf("item=search|type=k2") > -1) {
                if (s.aa_data.status.device != 3) {
                    target = "shp_pc_search_k2";
                } else {
                    target = "shp_sp_search_k2";
                }
            }
            
            if(target){
               s.eVar78 = target;
            }            
        }
        
        if(s.eVar78){
           s.eVar79 = "D=v78";
           s.eVar80 = "D=v78";
        }
        
        //[eVar81]  Y!money
        // ---------------------------------------------------------
        var ymoney_status = "g";
        if ("login" in s.aa_data.status && s.aa_data.status.login === true) {
            ymoney_status = "n";
            if("demographic" in s.aa_data && "ymoney" in s.aa_data.demographic && s.aa_data.demographic.ymoney !== "") {
                ymoney_status = s.aa_data.demographic.ymoney + '';
            }
        }
        s.eVar81 = ymoney_status;
        
        // [products] set products variable
        // ---------------------------------------------------------
        if ("products" in s.aa_data.status) {
            s.products = s.aa_data.status.products;
        }
        //
        // [event:purchase] Purchase
        // ---------------------------------------------------------
        if (s.section_object.subsection === "thank-you") {
            if (!("unique_item_count" in s.aa_data.status)) {
                s.aa_data.status.unique_item_count = 1;
            }
            
            var events_text_prefix = "purchase,event1";
            if ("delivery_quantity" in s.aa_data.status && s.aa_data.status.delivery_quantity >= 2) {
                events_text_prefix = "purchase=" + s.aa_data.status.delivery_quantity + ",event1:" + s.aa_data.status.purchaseID;
            } else {
                events_text_prefix = "purchase" + ",event1:" + s.aa_data.status.purchaseID;
            }
            s.events = s.apl(s.events, events_text_prefix + ",event17,event25=" + s.aa_data.status.unique_item_count, ",", 1);                

            // [event52] counter for webview purchace
            if(app_type){
                s.events = s.apl(s.events, "event52", ",", 1);
            }
            if ("coupon_type" in s.aa_data.status){
                if(s.aa_data.status.coupon_type && s.aa_data.status.coupon_type.indexOf("not use") === -1){
                    s.events = s.apl(s.events, "event74", ",", 1);
                }
                s.eVar50 = s.aa_data.status.coupon_type;
            }

            if ("products" in s.aa_data.status) {
                s.products = s.aa_data.status.products;
                var tmp_products_arr = s.products.split(','),
                    fixed_products = [],
                    eVar84 = "|eVar84=normal";
                if("delivery_quantity" in s.aa_data.status && s.aa_data.status.delivery_quantity >= 2) {
                    eVar84 = "|eVar84=gift";
                }
                for (var tmp_products_idx in tmp_products_arr) {
                    fixed_products.push(tmp_products_arr[tmp_products_idx] + eVar84);
                }
                s.products = fixed_products.join(',');
            }
            s.purchaseID = s.aa_data.status.purchaseID;
        }

        // [event:prodView,event13,event18] count product view
        //   set "prodView" in pageCode.
        // [eVar18] item categoryID
        // [eVar30] use storeLightTool (marchandising eVar)
        // ---------------------------------------------------------
        if (s.events && s.events.indexOf("prodView") > -1) {
            s.events = s.apl(s.events, "event13", ",", 1);
            s.events = s.apl(s.events, "event18", ",", 1);

            var arr_products_added_merchandise_evar = [];
            // set marchandising eVar
            if("use_store_lighttool" in s.aa_data.status && s.aa_data.status.use_store_lighttool === true){
                arr_products_added_merchandise_evar.push("eVar30=lighttool");
            }
            if("item_category" in s.aa_data.status && s.aa_data.status.item_category){
                arr_products_added_merchandise_evar.push("eVar18="+s.aa_data.status.item_category);
            }
            if(arr_products_added_merchandise_evar.length > 0){
                if(s.products.match(/;$/) === null){
                    s.products += ";;;;" + arr_products_added_merchandise_evar.join("|");
                }else{
                    s.products += ";;;" + arr_products_added_merchandise_evar.join("|");
                }
            }
        }

        // [event:scOpen] Cart list
        // ---------------------------------------------------------
        if (s.section_object.subsection === "cart-form" && !s.eo && !s.lnk) {
            s.events = s.apl(s.events, "scOpen", ",", 1);

            // if cookie has "sc_cartmethod" key, set eVar58(cart add method)
            if (s.c_r("sc_cartmethod") && s.c_r("sc_cartmethod") !== "done") {
                s.eVar58 = s.c_r("sc_cartmethod");
                s.c_w("sc_cartmethod", "done");
            } else if(!s.c_r("sc_cartmethod")) {
                s.eVar58 = "other";
            }
        }

        if (s.section_object.subsection === "order-form" && !s.eo && !s.lnk) {
            // if cookie has "sc_cartmethod" key, set eVar58(cart add method)
            if (s.c_r("sc_cartmethod") && s.c_r("sc_cartmethod") !== "done") {
                s.eVar58 = s.c_r("sc_cartmethod");
                s.c_w("sc_cartmethod", "done");
            } else if(!s.c_r("sc_cartmethod")) {
                s.eVar58 = "other";
            }

        }

        // [event:scCheckout] Start checkout
        // ---------------------------------------------------------
        if (s.section_object.subsection === "review-order" && !s.eo && !s.lnk) {
            s.events = s.apl(s.events, "scCheckout", ",", 1);
            // if cookie has "sc_cartmethod" key, set eVar58(cart add method)
            if (s.c_r("sc_cartmethod") && s.c_r("sc_cartmethod") !== "done") {
                s.eVar58 = s.c_r("sc_cartmethod");
                s.c_w("sc_cartmethod", "done");
            } else if(!s.c_r("sc_cartmethod")) {
                s.eVar58 = "other";
            }
        }


        // [event24] send review count
        // ---------------------------------------------------------
        if (s.pageName.indexOf("review:contribution:check:done") > -1) {
            s.events = s.apl(s.events, "event24", ",", 1);
        }

        // [event28] count show cartCoupon
        // ---------------------------------------------------------
        if ("cart_showCartCoupon" in s.aa_data.status){
            if(s.aa_data.status.cart_showCartCoupon == 1){
                s.events = s.apl(s.events, "event28", ",", 1);
            }
        }

        // [event29] count check cartCoupon
        // ---------------------------------------------------------
        if ("cart_checkCartCoupon" in s.aa_data.status){
            if(s.aa_data.status.cart_checkCartCoupon == 1){
                s.events = s.apl(s.events, "event29", ",", 1);
            }
        }

        // [event78] count couponObtain
        // ---------------------------------------------------------
        if(s.getQueryParam("cpnobtain") == 1){
            s.events = s.apl(s.events, "event78", ",", 1);
        }

        // Since addCart logic is planed to be removed from onclick link,
        // 'events' and 'product' shall be written on pagecode of cartlist page instead, only WHEN item is added.
        // We however would like to measure only when 'into cart' button is clicked on item page OR item option page,
        // BUT NOR added by favorite NOR added by purchase history,
        // so we make case like when 'events' value includes "scAdd" :
        // * MEASURE if refferer's section is "store"
        // * REMOVE (and NOT TO BE MEASURED) else
        if (s.events && s.events.indexOf("scAdd") > -1) {
            if (s.referrer_section && "section" in s.referrer_section &&
                 (s.referrer_section.section === "store" || s.referrer_section.section === "item_opt")) {
                s.c_w('sc_cartmethod', 'detail_cartAddButton', 0);
            }else{
                //remove events and products
                s.events = s.removeEvent(s.events, "scAdd");
                s.events = s.removeEvent(s.events, "event19");
                s.products = "";
            }
        }

        // variable set controll
        // don't double count "scCheckout"
        // ---------------------------------------------------------
        if (s.events && s.products) {
            if (s.events.indexOf("scCheckout") > -1) {
                s.chkstr = s.c_r("sc_chkstr");
                s.chkprod = s.products.split(";")[1];
                if (s.chkstr) {
                    s.chkarr = s.chkstr.split(",");
                    s.chkflg = false;
                    for (var j = 0; j < s.chkarr.length; j++) {
                        if (s.chkprod == s.chkarr[j]) {
                            s.chkflg = true;
                        }
                    }
                    if (!s.chkflg) {
                        s.wrtstr = s.chkstr + "," + s.chkprod;
                        s.c_w("sc_chkstr", s.wrtstr);
                    } else {
                        s.products = s.events = "";
                    }
                } else {
                    s.c_w("sc_chkstr", s.chkprod);
                }
            }
        }

        // vid from application
        //   use access from App
        // [eVar24] sp view
        // ---------------------------------------------------------
        if(!s.mcid && app_type){
            if (s.getQueryParam("appvi")) {
                s.new_vi_date=new Date;
                s.new_vi_date.setFullYear(s.new_vi_date.getFullYear() + 5);
                s.c_w("app_vi",s.getQueryParam("appvi"),s.new_vi_date);
                s.visitorID=s.c_r("app_vi");
                s.eVar24 = "D=c24";
            } else if (s.c_r("app_vi")) {
                s.visitorID=s.c_r("app_vi");
            }
        }

        // if click Ad, measurement tracking code
        // ---------------------------------------------------------
        if(s.linkObject) {
            var domainList=s.aa_pref.specific.tracking_ad_pattern.join("|");
            for(var i=0;i<domainList.length;i++) {
                if(s.linkObject.getAttribute("href") && s.linkObject.getAttribute("href").indexOf(domainList[i])>-1) {
                    if(s.linkObject.getAttribute("data-tcd")){
                        s.linkTrackVars = "eVar15,eVar28,eVar32,eVar35,eVar36";
                        s.eVar15 = "shp_pc_top_ad_" + s.linkObject.getAttribute("data-tcd");
                        s.eVar28 = "D=v15";
                        s.eVar32 = "D=v15";
                        s.eVar35 = "D=v15";
                        s.eVar36 = "D=v15";
                        s.linkType="o";
                        break;
                    }
                }
            }
        }
    }

    s.doPlugins = s_doPlugins;
    /************************** PLUGINS SECTION *************************/
    /*
     * Function - cookie domain function v0.1 for yj
     */
    s.c_gd = function() {
        var s = this,
            d = s.wd.location.hostname,
            n = s.fpCookieDomainPeriods,
            p;
        if (!n) {
            n = s.cookieDomainPeriods;
        }
        if (d) {
            n = n ? parseInt(n) : 2;
            n = n > 2 ? n : 2;
            p = d.lastIndexOf(".");
            if (p >= 0) {
                while (p >= 0 && n > 1) {
                    p = d.lastIndexOf(".", p - 1);
                    n--;
                }
                s.c_d = p > 0 && s.pt(d, ".", "c_gdf", 0) ? d.substring(p) : d;
            }
        }
        return s.c_d;
    };
    /*
     * Function - read combined cookies v 0.35 for yj
     */
    if (!s.__ccucr) {
        s.c_rr = s.c_r;
        s.__ccucr = true;

        function c_r(k) {
            var s = this,
                d = new Date,
                v = s.c_rr(k),
                c = s.c_rr('s_pers'),
                i, m, e;
            if (v) return v;
            k = s.ape(k);
            i = c.indexOf(' ' + k + '=');
            c = i < 0 ? s.c_rr('s_sess') : c;
            i = c.indexOf(' ' + k + '=');
            m = i < 0 ? i : c.indexOf('|', i);
            e = i < 0 ? i : c.indexOf(';', i);
            m = m > 0 ? m : e;
            v = i < 0 ? '' : s.epa(c.substring(i + 2 + k.length, m < 0 ? c.length : m));
            if (m > 0 && m != e)
                if (parseInt(c.substring(m + 1, e < 0 ? c.length : e)) < d.getTime()) {
                    d.setTime(d.getTime() - 60000);
                    s.c_w(s.epa(k), '', d);
                    v = '';
                }
            return v;
        }
        s.c_r = c_r;
    }
    /*
     * Function - write combined cookies v 0.35 for yj
     */
    if (!s.__ccucw) {
        s.c_wr = s.c_w;
        s.__ccucw = true;

        function c_w(k, v, e) {
            var s = this,
                d = new Date,
                ht = 0,
                pn = 's_pers',
                sn = 's_sess',
                pc = 0,
                sc = 0,
                pv, sv, c, i, t;
            d.setTime(d.getTime() - 60000);
            if (s.c_rr(k)) s.c_wr(k, '', d);
            k = s.ape(k);
            pv = s.c_rr(pn);
            i = pv.indexOf(' ' + k + '=');
            if (i > -1) {
                pv = pv.substring(0, i) + pv.substring(pv.indexOf(';', i) + 1);
                pc = 1;
            }
            sv = s.c_rr(sn);
            i = sv.indexOf(' ' + k + '=');
            if (i > -1) {
                sv = sv.substring(0, i) + sv.substring(sv.indexOf(';', i) + 1);
                sc = 1;
            }
            d = new Date;
            if (e) {
                if (e.getTime() > d.getTime()) {
                    pv += ' ' + k + '=' + s.ape(v) + '|' + e.getTime() + ';';
                    pc = 1;
                }
            } else {
                if (String(v).indexOf('%00') > -1) {
                    v = s.repl(v, '%00', '');
                }
                sv += ' ' + k + '=' + s.ape(v) + ';';
                sc = 1;
            } if (sc) {
                s.fpCookieDomainPeriods = "4";
                s.c_wr(sn, sv, 0);
            }
            if (pc) {
                t = pv;
                while (t && t.indexOf(';') != -1) {
                    var t1 = parseInt(t.substring(t.indexOf('|') + 1, t.indexOf(';')));
                    t = t.substring(t.indexOf(';') + 1);
                    ht = ht < t1 ? t1 : ht;
                }
                d.setTime(ht);
                s.fpCookieDomainPeriods = "4";
                s.c_wr(pn, pv, d);
            }
            return v == s.c_r(s.epa(k));
        }
        s.c_w = c_w;
    }

    /*
     * s.join: 1.0 - Joins an array into a string
     */
    s.join=new Function("v","p",""
    +"var s=this;var f,b,d,w;if(p){f=p.front?p.front:'';b=p.back?p.back:'"
    +"';d=p.delim?p.delim:'';w=p.wrap?p.wrap:'';}var str='';for(var x=0;x"
    +"<v.length;x++){if(typeof(v[x])=='object')str+=s.join(v[x],p);else s"
    +"tr+=w+v[x]+w;if(x<v.length-1)str+=d;}return f+str+b;");

    /*
     * Plugin: getQueryParam 2.4
     */
    s.getQueryParam = new Function("p", "d", "u", "h", "" + "var s=this,v='',i,j,t;d=d?d:'';u=u?u:(s.pageURL?s.pageURL:s.wd.loca" + "tion);if(u=='f')u=s.gtfs().location;while(p){i=p.indexOf(',');i=i<0" + "?p.length:i;t=s.p_gpv(p.substring(0,i),u+'',h);if(t){t=t.indexOf('#" + "')>-1?t.substring(0,t.indexOf('#')):t;}if(t)v+=v?d+t:t;p=p.substrin" + "g(i==p.length?i:i+1)}return v");
    s.p_gpv = new Function("k", "u", "h", "" + "var s=this,v='',q;j=h==1?'#':'?';i=u.indexOf(j);if(k&&i>-1){q=u.sub" + "string(i+1);v=s.pt(q,'&','p_gvf',k)}return v");
    s.p_gvf = new Function("t", "k", "" + "if(t){var s=this,i=t.indexOf('='),p=i<0?t:t.substring(0,i),v=i<0?'T" + "rue':t.substring(i+1);if(p.toLowerCase()==k.toLowerCase())return s." + "epa(v)}return''");
    /*
     * Plugin: getValOnce_v1.1
     */
    s.getValOnce = new Function("v", "c", "e", "t", "" + "var s=this,a=new Date,v=v?v:'',c=c?c:'s_gvo',e=e?e:0,i=t=='m'?6000" + "0:86400000;k=s.c_r(c);if(v){a.setTime(a.getTime()+e*i);s.c_w(c,v,e" + "==0?0:a);}return v==k?'':v");
    /*
     * Plugin: getPageName v2.1 - parse URL and return
     */
    s.getPageName = new Function("u", "" + "var s=this,v=u?u:''+s.wd.location,x=v.indexOf(':'),y=v.indexOf('/'," + "x+4),z=v.indexOf('?'),c=s.pathConcatDelim,e=s.pathExcludeDelim,g=s." + "queryVarsList,d=s.siteID,n=d?d:'',q=z<0?'':v.substring(z+1),p=v.sub" + "string(y+1,q?z:v.length);z=p.indexOf('#');p=z<0?p:s.fl(p,z);x=e?p.i" + "ndexOf(e):-1;p=x<0?p:s.fl(p,x);p+=!p||p.charAt(p.length-1)=='/'?s.d" + "efaultPage:'';y=c?c:'/';while(p){x=p.indexOf('/');x=x<0?p.length:x;" + "z=s.fl(p,x);if(!s.pt(s.pathExcludeList,',','p_c',z))n+=n?y+z:z;p=p." + "substring(x+1)}y=c?c:'?';while(g){x=g.indexOf(',');x=x<0?g.length:x" + ";z=s.fl(g,x);z=s.pt(q,'&','p_c',z);if(z){n+=n?y+z:z;y=c?c:'&'}g=g.s" + "ubstring(x+1)}return n");
    /*
     * Plugin: channelExtract : 1.0 - for yj
     */
    s.channelExtract = new Function("d", "p", "u", "pv", "" + "var s=this,v='';u=u?u:(s.pageURL?s.pageURL:s.wd.location);if(u=='f'" + ")u=s.gtfs().location;u=u+'';li=u.lastIndexOf(d);if(li>0){u=u.substr" + "ing(0,li);var i,n,a=s.split(u,d),al=a.length;if(al<p){if(pv==1) p=a" + "l;else return '';}for(i=0;i<p;i++){n=a[i];v=v+n+d;}return v.slice(0" + ",-1);}return '';");
    /*
     * Plugin: getNewRepeat 1.2 - Returns whether user is new or repeat
     */
    s.getNewRepeat = new Function("d", "cn", "" + "var s=this,e=new Date(),cval,sval,ct=e.getTime();d=d?d:30;cn=cn?cn:" + "'s_nr';e.setTime(ct+d*24*60*60*1000);cval=s.c_r(cn);if(cval.length=" + "=0){s.c_w(cn,ct+'-New',e);return'New';}sval=s.split(cval,'-');if(ct" + "-sval[0]<30*60*1000&&sval[1]=='New'){s.c_w(cn,ct+'-New',e);return'N" + "ew';}else{s.c_w(cn,ct+'-Repeat',e);return'Repeat';}");
    /*
     * Plugin: getTimeParting 2.0 for yj
     */
    s.getTimeParting = new Function("t", "z", "y", "l", "" + "var s=this,d,A,U,X,Z,W,B,C,D,Y;d=new Date();A=d.getFullYear();Y=U=S" + "tring(A);if(s.dstStart&&s.dstEnd){B=s.dstStart;C=s.dstEnd}else{;U=U" + ".substring(2,4);X='092925|102831|112730|122528|133127|143026|152925" + "|162730|172629|182828|193127';X=s.split(X,'|');for(W=0;W<=10;W++){Z" + "=X[W].substring(0,2);if(U==Z){B=X[W].substring(2,4);C=X[W].substrin" + "g(4,6)}}if(!B||!C){B='29';C='25'}B='03/'+B+'/'+A;C='10/'+C+'/'+A;}D" + "=new Date('1/1/2000');if(D.getDay()!=6||D.getMonth()!=0){return'Dat" + "a Not Available'}else{z=z?z:'0';z=parseFloat(z);B=new Date(B);C=new" + " Date(C);W=new Date();if(W>B&&W<C&&l!='0'){z=z+1}W=W.getTime()+(W.g" + "etTimezoneOffset()*60000);W=new Date(W+(3600000*z));X=['Sunday','Mo" + "nday','Tuesday','Wednesday','Thursday','Friday','Saturday'];B=W.get" + "Hours();C=W.getMinutes();D=W.getDay();Z=X[D];U='AM';A='Weekday';X='" + "00';if(C>45){X='45'}else if(C>30){X='30'}else if(C>15){X='15'}" + "if(B>=12){U='PM';B=B-12};if(B==0){B=12};if(D==6" + "||D==0){A='Weekend'}W=B+':'+X+U;if(y&&y!=Y){return'Data Not Availab" + "le'}else{if(t){if(t=='h'){return W}if(t=='d'){return Z}if(t=='w'){r" + "eturn A}}else{return Z+', '+W}}}");


    /*
     * Utility manageVars v1.4 - clear variable values (requires split 1.5)
     */
    s.manageVars=new Function("c","l","f",""
    +"var s=this,vl,la,vla;l=l?l:'';f=f?f:1 ;if(!s[c])return false;vl='pa"
    +"geName,purchaseID,channel,server,pageType,campaign,state,zip,events"
    +",products,transactionID';for(var n=1;n<76;n++){vl+=',prop'+n+',eVar"
    +"'+n+',hier'+n;}if(l&&(f==1||f==2)){if(f==1){vl=l;}if(f==2){la=s.spl"
    +"it(l,',');vla=s.split(vl,',');vl='';for(x in la){for(y in vla){if(l"
    +"a[x]==vla[y]){vla[y]='';}}}for(y in vla){vl+=vla[y]?','+vla[y]:'';}"
    +"}s.pt(vl,',',c,0);return true;}else if(l==''&&f==1){s.pt(vl,',',c,0"
    +");return true;}else{return false;}");
    s.lowercaseVars=new Function("t",""
    +"var s=this;if(s[t]&&t!='events'){s[t]=s[t].toString();if(s[t].index"
    +"Of('D=')!=0){s[t]=s[t].toLowerCase();}}");



    /*
      * Plugin Utility - RFL (remove from list) 1.0
      */
    s.rfl=new Function("l","v","d1","d2","ku",""
    +"var s=this,R=new Array(),C='',d1=!d1?',':d1,d2=!d2?',':d2,ku=!ku?0:"
    +"1;if(!l)return'';L=l.split(d1);for(i=0;i<L.length;i++){if(L[i].inde"
    +"xOf(':')>-1){C=L[i].split(':');C[1]=C[0]+':'+C[1];L[i]=C[0];}if(L[i"
    +"].indexOf('=')>-1){C=L[i].split('=');C[1]=C[0]+'='+C[1];L[i]=C[0];}"
    +"if(L[i]!=v&&C)R.push(C[1]);else if(L[i]!=v)R.push(L[i]);else if(L[i"
    +"]==v&&ku){ku=0;if(C)R.push(C[1]);else R.push(L[i]);}C='';}return s."
    +"join(R,{delim:d2})");

    /*
     * Plugin: getPreviousValue_v1.0 - return previous value of designated
     *   variable (requires split utility)
     */
    s.getPreviousValue=new Function("v","c","el",""
    +"var s=this,t=new Date,i,j,r='';t.setTime(t.getTime()+1800000);if(el"
    +"){if(s.events){i=s.split(el,',');j=s.split(s.events,',');for(x in i"
    +"){for(y in j){if(i[x]==j[y]){if(s.c_r(c)) r=s.c_r(c);v?s.c_w(c,v,t)"
    +":s.c_w(c,'no value',t);return r}}}}}else{if(s.c_r(c)) r=s.c_r(c);v?"
    +"s.c_w(c,v,t):s.c_w(c,'no value',t);return r}");

    /*
     * Utility Function: split v1.5 (JS 1.0 compatible)
     */
    s.split=new Function("l","d",""
    +"var i,x=0,a=new Array;while(l){i=l.indexOf(d);i=i>-1?i:l.length;a[x"
    +"++]=l.substring(0,i);l=l.substring(i+d.length);}return a");


    /*
     * Plugin: getPercentPageViewed v1.74
     */
    s.getPercentPageViewed=new Function("n",""
    +"var s=this,W=window,EL=W.addEventListener,AE=W.attachEvent,E=['load"
    +"','unload','scroll','resize','zoom','keyup','mouseup','touchend','o"
    +"rientationchange','pan'],K='s_ppv',P=K+'l',I=n||s.pageName||documen"
    +"t.location.href;W.s_Obj=s;if(!W.s_PPVevent){s.s_PPVg=function(n,o){"
    +"var c=s.c_r(o?P:K)||'',a=c.indexOf(',')>-1?c.split(',',10):[''],i;a"
    +"[0]=o?unescape(a[0]||''):I;for(i=1;i<9&&(i<a.length||!o);i++)a[i]=a"
    +"[i]?parseInt(a[i])||0:0;if(a.length>9||!o)a[9]=a[9]&&a[9]!='L'&&a[9"
    +"]!='LP'&&a[9]!='PL'?'P':a[9];return a};s.c_w(P,s.c_r(K)||'');s.c_w("
    +"K,escape(I)+',0,0,0,0,0,0,0,0');W.s_PPVevent=function(e){var W=wind"
    +"ow,D=document||{},B=D.body,E=D.documentElement||{},S=window.screen|"
    +"|{},Ho='offsetHeight',Hs='scrollHeight',Ts='scrollTop',Wc='clientWi"
    +"dth',Hc='clientHeight',M=Math,C=100,J='object',N='number',Z=',',s=W"
    +".s_Obj||W.s||0;e=e&&typeof e==J?e.type||'':'';if(!e.indexOf('on'))e"
    +"=e.substring(2);if(W.s_PPVt&&!e){clearTimeout(s_PPVt);s_PPVt=0}if(s"
    +"&&typeof s==J&&B&&typeof B==J){var h=M.max(B[Hs]||E[Hs],B[Ho]||E[Ho"
    +"],B[Hc]||E[Hc]||1),X=W.innerWidth||E[Wc]||B[Wc]||1,Y=W.innerHeight|"
    +"|E[Hc]||B[Hc]||1,x=S.width||1,y=S.height||1,r=M.round(C*(W.devicePi"
    +"xelRatio||1))/C,b=(D.pageYOffset||E[Ts]||B[Ts]||0)+Y,p=h>0&&b>0?M.r"
    +"ound(C*b/h):1,O=W.orientation,o=!isNaN(O)?M.abs(O)%180:Y>X?0:90,a=s"
    +".s_PPVg(n),L=(e=='load')||(a[1]<1),t,V=function(u,v,f,n){v=typeof v"
    +"!=N?u:v;v=f||(u>v)?u:v;return n?v:v>C?C:v<0?0:v};if(new RegExp('(iP"
    +"od|iPad|iPhone)').exec((window.navigator&&navigator.userAgent)||'')"
    +"&&o){t=x;x=y;y=t}o=o?'L':'P';a[9]=L||!a[9]?o:a[9].substring(0,1);if"
    +"(a[9]!='L'&&a[9]!='P')a[9]=o;s.c_w(K,escape(a[0])+Z+V(a[1],p,!L)+Z+"
    +"V(a[2],p,L)+Z+V(a[3],b,L,1)+Z+X+Z+Y+Z+x+Z+y+Z+r+Z+a[9]+(a[9]==o?'':"
    +"o))}if(!W.s_PPVt&&e!='unload')W.s_PPVt=setTimeout(W.s_PPVevent,333)"
    +"};for(var f=W.s_PPVevent,i=0;i<E.length;i++)if(EL)EL(E[i],f,false);"
    +"else if(AE)AE('on'+E[i],f);f()};var a=s.s_PPVg(n,1);return!argument"
    +"s.length||n=='-'?a[1]:a");

    /*
    * Plugin: getLoadTime
    */
    s.getLoadTime = function(){if(!window.s_loadT){var b=new Date().getTime();o=window.performance?performance.timing:0;a=o?o.requestStart:(window.inHeadTS||0);s_loadT=a?Math.round((b-a)/100):'';}return s_loadT;};


    /*
     * Plugin: roundPageName for yj
     */
    s.roundPageName = function(pn, rn) {
        var s = this;
        s.pnlidx = pn.lastIndexOf(":");
        s.retpn = pn.substring(0, s.pnlidx);
        s.retpn = s.retpn + ":" + rn;
        return s.retpn;
    };

    s.getSectionInfo = function (aa_pref, current_domain, current_path) {

        var current_domainandpath = "";
        var array_url_pathname = [];
        if (current_domain && current_path) {
            current_domainandpath = current_domain + current_path;
            array_url_pathname = current_path.split("/");
        } else {
            // default: current URL information
            current_domain = s.getLocation.hostname();
            current_path = s.getLocation.pathname();
            current_domainandpath = s.getLocation.hostname() + s.getLocation.pathname();
            array_url_pathname = s.getLocation.pathname().split("/");
        }

        // get subdomID
        var subdomID = null;
        if (aa_pref.specific.siteID[current_domain]) {
            subdomID = aa_pref.specific.siteID[current_domain];
        } else {
            if (current_domain.indexOf(".yahoo.co.jp") > -1) {
                subdomID = current_domain.replace(/\.yahoo\.co\.jp$/, "");
            } else {
                subdomID = "other";
            }
        }

        var array_section = [];
        var int_number_path_count = 0;
        var filename_reg = /(.*)(?:\.([^.]+$))/;
        var filename_match = null;
        var section_object = {};

        for (var tmp_i = 0, len_i = array_url_pathname.length; tmp_i < len_i; tmp_i++) {

            if (array_url_pathname[tmp_i] === "") {
                continue;
            }

            if (subdomID !== 'store' && array_url_pathname[tmp_i].match(/^[0-9]+$/) !== null) {
                int_number_path_count++;
                continue;
            }

            // remove ignore directory name
            if (array_url_pathname[tmp_i] === "cgi-bin") {
                continue;
            }

            // if path include extension ".", remove it
            filename_match = array_url_pathname[tmp_i].match(filename_reg);
            if (filename_match) {
                array_url_pathname[tmp_i] = filename_match[1];
            }
            array_section.push(array_url_pathname[tmp_i]);
        }
        // remove coupon id
        if(subdomID.indexOf("coupon") > -1){
            if(array_section.length === 0){
                array_section = ["coupon"];
            }else{
                /*
                 */                
                var path_len = document.location.pathname.split('/');
                var empty_check = "";
                
                if (path_len.length > 3) {
                    empty_check = path_len[3];
                }

                if (empty_check) {
                    array_section = ["coupon", array_section[1]];
                } else {
                    array_section = ["coupon"];
                }
            }
        }
        if(subdomID.indexOf("talk") > -1){
            if (array_section.length >= 2 && array_section[0] == "contact") {
                array_section = ["store", "talk", array_section[1]]
            }
            else if (array_section.length >= 1 && array_section[0] == "detail") {
                if ("talk_store_id" in s.aa_data.status && s.aa_data.status.talk_store_id) {
                    array_section = ["store", "talk", s.aa_data.status.talk_store_id]
                }
            }
        }

        switch (array_section.length) {
        case 0:
            section_object = {
                "section": "top",
                "subsection": "top",
                "only_subsection": "top"
            };
            break;
        case 1:
            section_object = {
                "section": array_section[0],
                "subsection": array_section[0],
                "only_subsection": ""
            };
            break;
        default:
            section_object = {
                "section": array_section[0],
                "subsection": array_section[0] + ":" + array_section[1],
                "only_subsection": array_section[1]
            };
        }

        // ---- make an exception of below subDomID pattern ------

        if(subdomID.indexOf("inform") > -1){
            section_object = {
                "section": "inform",
                "subsection": "inform",
                "only_subsection": "inform"
            };
        } else if(subdomID.indexOf("talk") > -1 && array_section.length > 2){
            section_object = {
                "section": array_section[0],
                "subsection": array_section[0] + ":" + array_section[2],
                "only_subsection": array_section[1]
            };
        }else if (section_object.section === "category") {
            // "category" expection handling
            // category:xxxxxx -> xxxxxx:list
            switch (section_object.only_subsection) {
            case "ranking":
                section_object.section = section_object.only_subsection;
                section_object.subsection = section_object.section + ":list";
                section_object.only_subsection = "list";
                break;
            case "brand":
                if (array_section.length >= 3 || int_number_path_count >= 3) {
                    section_object.subsection += ":" + "list";
                    section_object.only_subsection += ":" + "list";
                }
                break;
            case "":
                if (int_number_path_count >= 1) {
                    section_object.subsection = "category" + ":" + "list";
                    section_object.only_subsection = "list";
                }
                break;
            case "list":
                // this logic shall be removed, after the comfirmation
                // that the pathes are no longer expected to be updated here.
                if(array_section[array_section.length - 1] === "matome"){
                    section_object.section = "matome";
                    section_object.subsection = section_object.section + ":category";
                    section_object.only_subsection = "category";
                }
                break;
            default:
                break;
            }
        } else if (section_object.section === "ranking") {
            // "ranking" expection handling
            // ranking:xxxxxx(e.g. gender/cr_20) -> ranking:list
            if (section_object.only_subsection !== "" && section_object.only_subsection !== "category") {
                section_object.subsection = section_object.section;
                section_object.only_subsection = "";
            }
        } else if (section_object.section === "product") {
            if (array_section[1] === "j") {
                section_object.subsection = section_object.section + ":" + "jan";
                section_object.only_subsection = "jan";
            } else {
                section_object.subsection = section_object.section + ":" + array_section[2];
                section_object.only_subsection = array_section[2];
            }
        } else if (section_object.section === "products") {
            section_object.section = "product";
            if (section_object.only_subsection === "category" && array_section.length > 2) {
                section_object.subsection = section_object.section + ":category:"+array_section[2];
            } else {
                section_object.subsection = section_object.section + ":summary";
                section_object.only_subsection = "summary";
            }
        } else if (section_object.section === "item_opt"){
            section_object.subsection = section_object.section;
            section_object.only_subsection = section_object.section;
        } else if (section_object.section === "stores"){
            if(array_section[array_section.length - 1] === "matome"){
                section_object.section = "matome";
                section_object.subsection = section_object.section + ":stores";
                section_object.only_subsection = "stores";
            }
        }else if(section_object.section === "search"){
            section_object.subsection = section_object.section;
            section_object.only_subsection = section_object.section;
        }else if(section_object.section === "matome"){
            // for matome URL reconstruction
            if(section_object.only_subsection === "tag"){
                section_object.subsection = section_object.section + ":tags";
                section_object.only_subsection = "tags";
            }else if(section_object.only_subsection === "search"){
                section_object.subsection = section_object.section + ":freeword";
                section_object.only_subsection = "freeword";
            }else if(section_object.only_subsection === "article"){
                section_object.subsection = section_object.section + ":category";
                section_object.only_subsection = "category";
            }else if(section_object.only_subsection === "list" && array_section.length >= 3){
                section_object.subsection = section_object.section + ":" + array_section[2];
                section_object.only_subsection = array_section[2];
            }
        }

        // "campaign" expection handling
        var arr_campaign_matching = aa_pref.specific.campaign_matching;
        for (var i_campaign = 0, len_campaign = arr_campaign_matching.length; i_campaign < len_campaign; i_campaign++) {
            if (current_domainandpath.indexOf(arr_campaign_matching[i_campaign]) === 0) {
                section_object = {
                    "section": "campaign",
                    "subsection": "campaign",
                    "only_subsection": ""
                };
            }
        }

        // "brand" exception handling
        if (current_domainandpath.match(/^shopping\.yahoo\.co\.jp\/brand\/([0-9]+?)\/?$/)) {
            section_object = {
                "section": "brand",
                "subsection": "brand:detail",
                "only_subsection": "detail"
            };
        }

        // "category.shopping.yahoo.co.jp" exception handling
        if (current_domainandpath.indexOf("category.shopping.yahoo.co.jp") > -1) {
            section_object.subsection = "category" + ":" + section_object.section;
            section_object.only_subsection = section_object.section;
            section_object.section = "category";
        }

        // "store.shopping.yahoo.co.jp" exception handling
        if (current_domainandpath.indexOf("store.shopping.yahoo.co.jp") > -1 || current_domainandpath.indexOf("store.adult.shopping.yahoo.co.jp") > -1) {
            section_object.subsection = "store" + ":" + section_object.section;
            section_object.only_subsection = section_object.section;
            section_object.section = "store";
        }

        section_object.path_arr = array_section;

        /*
         */
        // if url include ";_ylt=" remove it.
        var matching_section_ylt    = section_object.section.match(/^(.*?);_ylt=.*$/);
        var matching_subsection_ylt = section_object.subsection.match(/^(.*?);_ylt=.*$/);

        if (matching_section_ylt && matching_section_ylt[1]) {
            section_object.section = matching_section_ylt[1];
        }

        if (matching_subsection_ylt && matching_subsection_ylt[1]) {
            section_object.subsection = matching_subsection_ylt[1];
        }

        return section_object;


    };

    /**
     * generate Section Name String
     *
     * @param {String} sectionname : natural section name
     * @param {String} sectionname_prefix : section name prefix
     * @param {String} prefix : common name prefix
     * @return {String} SectionName
     **/
    s.generateSectionNameString = function(sectionname, sectionname_prefix, prefix) {

        if (s.sectionname_prefix === "sp") {
            return prefix + ":" + sectionname_prefix + "_" + sectionname;
        } else {
            return prefix + ":" + sectionname;
        }

    };
    /**
     * decide whether it is in the store area or not by given parameters
     * @param {type} subDomID
     * @param {type} section
     * @param {type} only_subsection (not in use so far)
     * @returns {Boolean} it is in the store area if true, not otherwise.
     **/
    s.isSubsectionStore = function(subDomID, section, only_subsection){
        var store_area_settings = [["store","store"], ["store.adult","store"], ["main", "storestamprally"], ["triple"]];
        return s.isSubsection(store_area_settings, subDomID, section, only_subsection);
    }
    /**
     * decide whether it is in the area or not by given parameters
     * @param {type} area_settings array of arrays [[subDomID, section], [subDomID]]
     * @param {type} subDomID
     * @param {type} section
     * @param {type} only_subsection (not in use so far)
     * @returns {Boolean} it is in the store area if true, not otherwise.
     **/
    s.isSubsection = function(area_settings, subDomID, section, only_subsection){
        if(subDomID.length > 3 && subDomID.substring(0, 3) === "sp_"){
            subDomID = subDomID.substring(3, subDomID.length);
        }
        for(i = 0;i < area_settings.length;i++){
            if(area_settings[i].length === 2 && area_settings[i][0] === subDomID && area_settings[i][1] === section){
                return true;
            }else if(area_settings[i].length === 1 && area_settings[i][0] === subDomID){
                return true;
            }
        }
        return false;
    };
    /**
     * manage history cooprating with cookie
     * @param {string} key
     * @param {integer} max_size
     * @param {string} splitter
     * @param {string} latest (value to append)
     * @returns {Boolean} it is in the store area if true, not otherwise.
     **/
    s.manageHistory = function(key, max_size, splitter, latest) {
        //check parameter latest
        if(!latest || latest.length === 0){
            return null;
        }
        //retrieve last value from cookie
        last_value = s.c_r(key);
        //split into array
        var history_array = !last_value || last_value.length === 0?[]:last_value.split(splitter);
        //if last and latest are equal, do nothing
        if(history_array.length >= 1 && history_array[history_array.length - 1] === latest){
            return last_value;
        }
        //remove older and make a room for latest
        while(history_array.length >= max_size){
            history_array.shift();
        }
        //add latest to the tail
        history_array.push(latest);
        //create current value
        current_value = history_array.join(splitter);
        //put into cookie
        s.c_w(key, current_value);
        return current_value;
    };
    /*
     * safe event remove function
     * @param {type} events : subject
     * @param {type} remove : event to be removed
     * @returns {String}
     */
    s.removeEvent = function(events, remove){
        events = events.replace(/\s+/g, "");
        splits = events.split(",");
        removed = [];
        for(i = 0;i < splits.length;i++){
            if(splits[i] === remove){
                continue;
            }
            removed.push(splits[i]);
        }
        return removed.join(",");
    };
    /*
     * getInsideStoreOrder()
     * to get a customized list order for inside store search
     */
    s.getInsideStoreOrder = function(order_value) {
        if(!order_value){
            order_value = "0";
        }
        if(order_value !== "0" || !("store_list_model" in s.aa_data.status)){
            return order_value;
        }
        return s.aa_data.status.store_list_model === "default"?"97":
               s.aa_data.status.store_list_model === "custom" ?"98":"0";
    };
    /**
     * returns whet
     */
    s.existsParam = function(parameter) {
        if(!window.location.search || window.location.search.length == 0){
            return false;
        }
        var parameter_array = window.location.search.substring(1).split("&");
        for(var count = 0;count < parameter_array.length;count++){
            if(!parameter_array[count]){
                continue;
            }
            var param_value = parameter_array[count].split("=");
            if(param_value[0] !== parameter){
                continue;
            }
            return param_value.length >= 2 && param_value[1] && param_value[1].length > 0;
        }
        return false;
    };
    /*
     * Plugin: storeSearchParams for yj
     */
    s.storeSearchParams = function() {
        var store_search_params = ["cid", "pf", "pt", "view", "uIv", "uSa", "uFs", "uKy", "uAs", "uNe", "uUs", "ea", "n", "X", "ship", "strcid", "brandid", "p"];
        var store_search_conditions = [];
        for(var i = 0;i < store_search_params.length;i++){
            var parameter = store_search_params[i];
            if(parameter === "p"){
                store_search_value = s.existsParam("p")?"1":"0";
            }else{
                var store_search_value = s.getQueryParam(parameter);
                if(parameter === "X"){
                    store_search_value = s.getInsideStoreOrder(store_search_value);
                }else if(!store_search_value){
                    store_search_value = "";
                }
            }
            store_search_conditions.push(parameter + "=" + store_search_value);

        }
        return store_search_conditions.join("/");
    };
    /*
     * Plugin clickStoreSearch()
     */
    s.clickStoreSearch = function() {
        s.linkTrackVars = "eVar45";
        s.eVar45 = s.storeSearchParams();

        // take p value for eVar46 only when sp(it fails for pc since it is encoded in EUC-JP)
        if (s.sectionname_prefix === "sp") {
            var keyword = s.getQueryParam("p");
            if(keyword){
                s.linkTrackVars += ",eVar46";
                s.eVar46 = keyword;
            }
        }

        s.tl(true, 'o', 'clickStoreSearch ');
    };
    /*
     * Plugin clickCoupon()
     */
    s.clickCoupon = function(type) {
        var event =
            type === "get_store_all"?"event70":
            type === "get_one"      ?"event71":
            type === "use_one"      ?"event72":null;
        if(event){
            s.linkTrackVars = "events";
            s.linkTrackEvents = event;
            s.events = event;
            s.tl(true, 'o', 'clickCoupon');
        }
    };
   /*
    * Plugin clickPage()
    */
   s.clickPage = function(param_name) {
       var page = s.getQueryParam(param_name);
       if(!page){
           page = "0";
       }
       s.tl(true, 'o', 'clickPage');
   };
    /*
     * Plugin addCart v0.1 for yj
     */
    s.addCart = function(prd, abtest) {
        var s = this;
        s.products = prd;
        s.linkTrackVars = "products,events";
        s.linkTrackEvents = "scAdd,event19";
        s.events = "scAdd,event19";
        s.c_w('sc_cartmethod', 'detail_cartAddButton', 0);

        s.tl(true, 'o', 'addCart');
    };

    /**
     * DEACTIVATED : measure "Add Favorite" action
     *
     * @param {String} type : Favorite Type(store / item)
     * @param {String} id : Favorite ID(StoreID)
     **/
    s.addFavorite = function(type, id) {
    };

    /*
     * Plugin addShopAround()
     */
    s.addShopAround = function() {
        var s = this;
        s.linkTrackVars = "";
        s.linkTrackEvents = "";
        s.c_w('sc_cartmethod', 'thankyou', 0);

        s.tl(true, 'o', 'addShopAround');
    };

    /*
     * Plugin clickTrackingCode()
     */
    s.clickTrackingCode = function(code) {
        s.linkTrackVars = "eVar15,eVar28,eVar36";
        s.eVar15 = code;
        s.eVar28 = "D=v15";
        s.eVar36 = "D=v15";
        s.tl(true, 'o', 'clickTrackingCode');
    };
    /*
     * Plugin reviewSubmitted v0.1 for yj
     */
    s.reviewSubmitted = function(review_type, submit_type) {
        var review_event =
            review_type === 'store'?
                submit_type === 'star'     ?'event40':
                submit_type === 'text_star'?'event40,event41':null:
            review_type === 'item' ?
                submit_type === 'star'     ?'event42':
                submit_type === 'text_star'?'event42,event43':null:null;
        if(review_event){
            var s = this;
            s.linkTrackVars = 'events';
            s.linkTrackEvents = review_event;
            s.events = review_event;
            s.tl(true, 'o', 'reviewSubmitted');
        }
    };
    /*
     * Plugin clickCampaignButton v0.1 for yj
     */
    s.clickCampaignButton = function(name) {
        var s = this;
        s.linkTrackVars = 'prop41';
        s.prop41 = name;
        s.tl(true, 'o', 'clickCampaignButton');
    };
    /*
     * Plugin submitEvents v0.1 for yj
     */
    s.submitEvents = function(events) {
        if(events){
            var s = this;
            s.linkTrackVars = 'events';
            s.linkTrackEvents = events;
            s.events = events;
            s.tl(true, 'o', 'submitEvents');
        }
    };
    /*
     * Plugin clickRelatedKeywordLink()
     */
    s.clickRelatedKeywordLink = function(area,type) {

        type = type ? type : 'search';

        // write cookie
        s.c_w(type + "_related_pos",area,0);
    };

    /*
     * Plugin clickDYMLink()
     */
    s.clickDYMLink = function(){

        // write cookie
        s.c_w("search_dym_link","search",0);
    };

    /*
     * Plugin clickSearchResultPos()
     */
    s.clickSearchResultPos = function(position) {

        // write cookie
        s.c_w("search_result_pos",position,0);
    };

    /*
     * Plugin clickBreadcrumbLink()
     *
     */
    s.clickBreadcrumbLink = function(name) {

        // write cookie
        s.c_w("breadcrumb","category=" + name + "|refer=" + s.prop3,0);
    };

    /*
     * Plugin clickBuyerCancel
     *
     */
    s.clickBuyerCancel = function(sellerId) {
        YAHOO.JP.sc.linkTrackVars = "products,events";
        YAHOO.JP.sc.linkTrackEvents = "event77";
        YAHOO.JP.sc.events = "event77";
        YAHOO.JP.sc.products = ";"+ sellerId +";;;event77=1;";
        YAHOO.JP.sc.tl(this, 'o', 'cancelPurchase');
    };

    /*
     * Plugin clickStoreItemQandaAns 
     *
     */
    s.clickStoreItemQandaAns = function(sellerId) {
        YAHOO.JP.sc.linkTrackVars = "products,events";
        YAHOO.JP.sc.linkTrackEvents = "event86";
        YAHOO.JP.sc.events = "event86";
        YAHOO.JP.sc.products = ";"+ sellerId +";;;event86=1;";
        YAHOO.JP.sc.tl(this, 'o', 'getStoreItemQandA_Ans');
    };

    /*
     * Plugin clickStoreItemQandaNoAns 
     *
     */
    s.clickStoreItemQandaNoAns = function(sellerId) {
        YAHOO.JP.sc.linkTrackVars = "products,events";
        YAHOO.JP.sc.linkTrackEvents = "event87";
        YAHOO.JP.sc.events = "event87";
        YAHOO.JP.sc.products = ";"+ sellerId +";;;event87=1;";
        YAHOO.JP.sc.tl(this, 'o', 'getStoreItemQandA_NoAns');
    };

    /**
     * plugin clickTrackUtil()
     * @param {Object|Boolean} clickObject required:(if can't get click object, set true)
     * @param {String} trackingName required:(for use customLinkName)
     * @param {String} trackingVariable option
     * @param {String} trackingEvent option
     **/
     s.clickTrackUtil = function(element,trackingName,trackingVariable,trackingEvent){

      if(arguments.length === 0){
        return null;
      }
      if(!trackingName){
        return null;
      }
      var arr_track_variable = [];
      var arr_track_events = [];
      var linkName = trackingName;

      switch(linkName){
        case "addFavorite":
          (trackingEvent) ? trackingEvent += ",event73" : trackingEvent = "event73";
          linkName = "clickTrackUtil_" + trackingName;
        break;


        case "getCoupon":
          (trackingEvent) ? trackingEvent += ",event71" : trackingEvent = "event71";
          linkName = "clickTrackUtil_" + trackingName;
        break;

        default:
        break;
      }

      // added custom track variable and events
      if(trackingVariable && trackingVariable.indexOf("event") === -1){
        arr_track_variable = arr_track_variable.concat(trackingVariable.split(","));
      }
      if(trackingVariable && trackingVariable.indexOf("event") > -1){
        arr_track_events = arr_track_variable.concat(trackingVariable.split(","));
      }
      if(trackingEvent && trackingEvent.indexOf("event") > -1){
        arr_track_events = arr_track_events.concat(trackingEvent.split(","));
      }
      s.events = arr_track_events.join(",");

      if(linkName == ""){
        return null;
      }

      // [eVar27] subsection when clickTracking action occured.
      s.eVar27 = s.prop8;
      arr_track_variable.push("eVar27");

      // if linkTrackEvents is existed, added linkTrackVars to "events";
      if(arr_track_events && arr_track_events.length >= 1){
        arr_track_variable.push("events");
      }

      // finalize and track custom link
      s.linkTrackVars = arr_track_variable.join(",");
      s.linkTrackEvents = arr_track_events.join(",");

      s.tl(element,"o",linkName);
     }


    /*
     * Plugin: convHtoF for yj
     * convert halfwidth Kana to fullwidth Kana
     */
    s.convHtoF = function(v) {
        var str = v;
        var Kana1 = new Array("\uff76\uff9e", "\uff77\uff9e", "\uff78\uff9e", "\uff79\uff9e", "\uff7a\uff9e", "\uff7b\uff9e", "\uff7c\uff9e", "\uff7d\uff9e", "\uff7e\uff9e", "\uff7f\uff9e", "\uff80\uff9e", "\uff81\uff9e",
            "\uff82\uff9e", "\uff83\uff9e", "\uff84\uff9e", "\uff8a\uff9e", "\uff8b\uff9e", "\uff8c\uff9e", "\uff8d\uff9e", "\uff8e\uff9e", "\uff8a\uff9f", "\uff8b\uff9f", "\uff8c\uff9f", "\uff8d\uff9f", "\uff8e\uff9f", "\uff66", "\uff67",
            "\uff68", "\uff69", "\uff6a", "\uff6b", "\uff6c", "\uff6d", "\uff6e", "\uff6f", "\uff70", "\uff71", "\uff72", "\uff73", "\uff74", "\uff75", "\uff76", "\uff77", "\uff78", "\uff79",
            "\uff7a", "\uff7b", "\uff7c", "\uff7d", "\uff7e", "\uff7f", "\uff80", "\uff81", "\uff82", "\uff83", "\uff84", "\uff85", "\uff86", "\uff87", "\uff88", "\uff89", "\uff8a", "\uff8b",
            "\uff8c", "\uff8d", "\uff8e", "\uff8f", "\uff90", "\uff91", "\uff92", "\uff93", "\uff94", "\uff95", "\uff96", "\uff97", "\uff98", "\uff99", "\uff9a", "\uff9b", "\uff9c", "\uff9d");
        var Kana2 = new Array("\u30ac", "\u30ae", "\u30b0", "\u30b2", "\u30b4", "\u30b6", "\u30b8", "\u30ba", "\u30bc", "\u30be", "\u30c0", "\u30c2",
            "\u30c5", "\u30c7", "\u30c9", "\u30d0", "\u30d3", "\u30d6", "\u30d9", "\u30dc", "\u30d1", "\u30d4", "\u30d7", "\u30da", "\u30dd", "\u30f2", "\u30a1",
            "\u30a3", "\u30a5", "\u30a7", "\u30a9", "\u30e3", "\u30e5", "\u30e7", "\u30c3", "\u30fc", "\u30a2", "\u30a4", "\u30a6", "\u30a8", "\u30aa", "\u30ab",
            "\u30ad", "\u30af", "\u30b1", "\u30b3", "\u30b5", "\u30b7", "\u30b9", "\u30bb", "\u30bd", "\u30bf", "\u30c1", "\u30c4", "\u30c6", "\u30c8", "\u30ca",
            "\u30cb", "\u30cc", "\u30cd", "\u30ce", "\u30cf", "\u30d2", "\u30d5", "\u30d8", "\u30db", "\u30de", "\u30df", "\u30e0", "\u30e1", "\u30e2", "\u30e4",
            "\u30e6", "\u30e8", "\u30e9", "\u30ea", "\u30eb", "\u30ec", "\u30ed", "\u30ef", "\u30f3");
        while (str.match(/[\uff66-\uff9d]/)) {
            for (var i = 0; i < Kana1.length; i++) {
                str = str.replace(Kana1[i], Kana2[i]);
            }
        }
        return str;
    };

    /*** Utility Functions ***/
    /*
     * Utility Function: p_c
     */
    s.p_c = new Function("v", "c", "" + "var x=v.indexOf('=');return c.toLowerCase()==v.substring(0,x<0?v.le" + "ngth:x).toLowerCase()?v:0");
    /*
     * Plugin Utility: apl v1.1
     */
    s.apl = new Function("l", "v", "d", "u", "" + "var s=this,m=0;if(!l)l='';if(u){var i,n,a=s.split(l,d);for(i=0;i<a." + "length;i++){n=a[i];m=m||(u==1?(n==v):(n.toLowerCase()==v.toLowerCas" + "e()));}}if(!m)l=l?l+d+v:v;return l");

    s.isArray = function(o) {
        return Object.prototype.toString.call(o) === '[object Array]';
    };

    s.getReferrer = function() {
        return s.wd.document.referrer;
    };
    s.getLocation = {
        hostname: function() {
            /*
             */
            if (s.wd.location.href.indexOf("shopping.yahoo.co.jp/coupon") > -1) {
                return "coupon.shopping.yahoo.co.jp";
            } else if (s.wd.location.href.indexOf("shopping.yahoo.co.jp/search") > -1) {
                return "search.shopping.yahoo.co.jp";                
            } else {
                return s.wd.location.hostname;
            }
        },
        pathname: function() {
            var matching_url_ylc = s.wd.location.pathname.match(/^(.+?);_ylc=.+?(?:\?(.*))?$/);
            var matching_url_ylt = s.wd.location.pathname.match(/^(.+?);_ylt=.+?(?:\?(.*))?$/);

            if(matching_url_ylc && matching_url_ylc[1]){
                return matching_url_ylc[1];
            }else if(matching_url_ylt && matching_url_ylt[1]) {
                return matching_url_ylt[1];
            }else{
                return s.wd.location.pathname;
            }
        },
        href: function() {
            var matching_url_ylc = s.wd.location.href.match(/^(.+?);_ylc=.+?\?(.*)$/);
            var matching_url_ylt = s.wd.location.href.match(/^(.+?);_ylt=.+?\?(.*)$/);

            if(matching_url_ylc && matching_url_ylc[1] && matching_url_ylc[2]){
                return matching_url_ylc[1] + "?" + matching_url_ylc[2];
            }else if(matching_url_ylt && matching_url_ylt[1] && matching_url_ylt[2]){
                return matching_url_ylt[1] + "?" + matching_url_ylt[2];
            }else{
                return s.wd.location.href;
            }
        }
    };

	/*
	 * Utility: AppMeasurement Compatibility v1.1
	 * Define deprecated H-code s properties and methods used by legacy plugins
	 */
	s.wd=window;
	s.fl=new Function("x","l",""
	+"return x?(''+x).substring(0,l):x");
	s.pt=new Function("x","d","f","a",""
	+"var s=this,t=x,z=0,y,r,l='length';while(t){y=t.indexOf(d);y=y<0?t[l"
	+"]:y;t=t.substring(0,y);r=s[f](t,a);if(r)return r;z+=y+d[l];t=x.subs"
	+"tring(z,x[l]);t=z<x[l]?t:''}return''");
	s.rep=new Function("x","o","n",""
	+"var a=new Array,i=0,j;if(x){if(x.split)a=x.split(o);else if(!o)for("
	+"i=0;i<x.length;i++)a[a.length]=x.substring(i,i+1);else while(i>=0){"
	+"j=x.indexOf(o,i);a[a.length]=x.substring(i,j<0?x.length:j);i=j;if(i"
	+">=0)i+=o.length}}x='';j=a.length;if(a&&j>0){x=a[0];if(j>1){if(a.joi"
	+"n)x=a.join(n);else for(i=1;i<j;i++)x+=n+a[i]}}return x");
	s.ape=new Function("x",""
	+"var s=this,h='0123456789ABCDEF',f='+~!*()\\'',i,c=s.charSet,n,l,e,y"
	+"='';c=c?c.toUpperCase():'';if(x){x=''+x;if(s.em==3){x=encodeURIComp"
	+"onent(x);for(i=0;i<f.length;i++){n=f.substring(i,i+1);if(x.indexOf("
	+"n)>=0)x=s.rep(x,n,'%'+n.charCodeAt(0).toString(16).toUpperCase())}}"
	+"else if(c=='AUTO'&&('').charCodeAt){for(i=0;i<x.length;i++){c=x.sub"
	+"string(i,i+1);n=x.charCodeAt(i);if(n>127){l=0;e='';while(n||l<4){e="
	+"h.substring(n%16,n%16+1)+e;n=(n-n%16)/16;l++}y+='%u'+e}else if(c=='"
	+"+')y+='%2B';else y+=escape(c)}x=y}else x=s.rep(escape(''+x),'+','%2"
	+"B');if(c&&c!='AUTO'&&s.em==1&&x.indexOf('%u')<0&&x.indexOf('%U')<0)"
	+"{i=x.indexOf('%');while(i>=0){i++;if(h.substring(8).indexOf(x.subst"
	+"ring(i,i+1).toUpperCase())>=0)return x.substring(0,i)+'u00'+x.subst"
	+"ring(i);i=x.indexOf('%',i)}}}return x");
	s.epa=new Function("x",""
	+"var s=this,y,tcf;if(x){x=s.rep(''+x,'+',' ');if(s.em==3){tcf=new Fu"
	+"nction('x','var y,e;try{y=decodeURIComponent(x)}catch(e){y=unescape"
	+"(x)}return y');return tcf(x)}else return unescape(x)}return y");
	s.parseUri=new Function("u",""
	+"if(u){u=u+'';u=u.indexOf(':')<0&&u.indexOf('//')!=0?(u.indexOf('/')"
	+"==0?'/':'//')+u:u}u=u?u+'':window.location.href;var e,a=document.cr"
	+"eateElement('a'),l=['href','protocol','host','hostname','port','pat"
	+"hname','search','hash'],p,r={href:u,toString:function(){return this"
	+".href}};a.setAttribute('href',u);for(e=1;e<l.length;e++){p=l[e];r[p"
	+"]=a[p]||''}delete a;p=r.pathname||'';if(p.indexOf('/')!=0)r.pathnam"
	+"e='/'+p;return r");
	s.gtfs=new Function(""
	+"var w=window,l=w.location,d=document,u;if(!l.origin)l.origin=l.prot"
	+"ocol+'//'+l.hostname+(l.port?':'+l.port:'');u=l!=w.parent.location?"
	+"d.referrer:d.location;return{location:s.parseUri(u)}");

    /* WARNING: Changing any of the below variables will cause drastic
changes to how your visitor data is collected.  Changes should only be
made when instructed to do so by your account manager.*/
    if (!s.visitor && "visitorID" in YAHOO.JP.shp.data.status && YAHOO.JP.shp.data.status.visitorID) {
    s.visitorNamespace = "yahoojapan";
    s.trackingServer = "yahoojapan.d1.sc.omtrdc.net";
    } else {
        s.visitorNamespace = "yahoojapanshopping";
        s.trackingServer = "yahoojapanshopping.sc.omtrdc.net";
    }

	// copy object
	YAHOO.JP.sc = s;

})();

/*
 ============== DO NOT ALTER ANYTHING BELOW THIS LINE ! ===============

AppMeasurement for JavaScript version: 2.9.0
Copyright 1996-2016 Adobe, Inc. All Rights Reserved
More info available at http://www.adobe.com/marketing-cloud.html
*/
function AppMeasurement(r){var a=this;a.version="2.9.0";var k=window;k.s_c_in||(k.s_c_il=[],k.s_c_in=0);a._il=k.s_c_il;a._in=k.s_c_in;a._il[a._in]=a;k.s_c_in++;a._c="s_c";var p=k.AppMeasurement.Mb;p||(p=null);var n=k,m,s;try{for(m=n.parent,s=n.location;m&&m.location&&s&&""+m.location!=""+s&&n.location&&""+m.location!=""+n.location&&m.location.host==s.host;)n=m,m=n.parent}catch(u){}a.D=function(a){try{console.log(a)}catch(b){}};a.Ga=function(a){return""+parseInt(a)==""+a};a.replace=function(a,b,d){return!a||
0>a.indexOf(b)?a:a.split(b).join(d)};a.escape=function(c){var b,d;if(!c)return c;c=encodeURIComponent(c);for(b=0;7>b;b++)d="+~!*()'".substring(b,b+1),0<=c.indexOf(d)&&(c=a.replace(c,d,"%"+d.charCodeAt(0).toString(16).toUpperCase()));return c};a.unescape=function(c){if(!c)return c;c=0<=c.indexOf("+")?a.replace(c,"+"," "):c;try{return decodeURIComponent(c)}catch(b){}return unescape(c)};a.tb=function(){var c=k.location.hostname,b=a.fpCookieDomainPeriods,d;b||(b=a.cookieDomainPeriods);if(c&&!a.ya&&!/^[0-9.]+$/.test(c)&&
(b=b?parseInt(b):2,b=2<b?b:2,d=c.lastIndexOf("."),0<=d)){for(;0<=d&&1<b;)d=c.lastIndexOf(".",d-1),b--;a.ya=0<d?c.substring(d):c}return a.ya};a.c_r=a.cookieRead=function(c){c=a.escape(c);var b=" "+a.d.cookie,d=b.indexOf(" "+c+"="),f=0>d?d:b.indexOf(";",d);c=0>d?"":a.unescape(b.substring(d+2+c.length,0>f?b.length:f));return"[[B]]"!=c?c:""};a.c_w=a.cookieWrite=function(c,b,d){var f=a.tb(),e=a.cookieLifetime,g;b=""+b;e=e?(""+e).toUpperCase():"";d&&"SESSION"!=e&&"NONE"!=e&&((g=""!=b?parseInt(e?e:0):-60)?
(d=new Date,d.setTime(d.getTime()+1E3*g)):1==d&&(d=new Date,g=d.getYear(),d.setYear(g+5+(1900>g?1900:0))));return c&&"NONE"!=e?(a.d.cookie=a.escape(c)+"="+a.escape(""!=b?b:"[[B]]")+"; path=/;"+(d&&"SESSION"!=e?" expires="+d.toUTCString()+";":"")+(f?" domain="+f+";":""),a.cookieRead(c)==b):0};a.qb=function(){var c=a.Util.getIeVersion();"number"===typeof c&&10>c&&(a.unsupportedBrowser=!0,a.fb(a,function(){}))};a.fb=function(a,b){for(var d in a)a.hasOwnProperty(d)&&"function"===typeof a[d]&&(a[d]=b)};
a.L=[];a.ba=function(c,b,d){if(a.za)return 0;a.maxDelay||(a.maxDelay=250);var f=0,e=(new Date).getTime()+a.maxDelay,g=a.d.visibilityState,h=["webkitvisibilitychange","visibilitychange"];g||(g=a.d.webkitVisibilityState);if(g&&"prerender"==g){if(!a.ca)for(a.ca=1,d=0;d<h.length;d++)a.d.addEventListener(h[d],function(){var c=a.d.visibilityState;c||(c=a.d.webkitVisibilityState);"visible"==c&&(a.ca=0,a.delayReady())});f=1;e=0}else d||a.o("_d")&&(f=1);f&&(a.L.push({m:c,a:b,t:e}),a.ca||setTimeout(a.delayReady,
a.maxDelay));return f};a.delayReady=function(){var c=(new Date).getTime(),b=0,d;for(a.o("_d")?b=1:a.qa();0<a.L.length;){d=a.L.shift();if(b&&!d.t&&d.t>c){a.L.unshift(d);setTimeout(a.delayReady,parseInt(a.maxDelay/2));break}a.za=1;a[d.m].apply(a,d.a);a.za=0}};a.setAccount=a.sa=function(c){var b,d;if(!a.ba("setAccount",arguments))if(a.account=c,a.allAccounts)for(b=a.allAccounts.concat(c.split(",")),a.allAccounts=[],b.sort(),d=0;d<b.length;d++)0!=d&&b[d-1]==b[d]||a.allAccounts.push(b[d]);else a.allAccounts=
c.split(",")};a.foreachVar=function(c,b){var d,f,e,g,h="";e=f="";if(a.lightProfileID)d=a.P,(h=a.lightTrackVars)&&(h=","+h+","+a.ga.join(",")+",");else{d=a.g;if(a.pe||a.linkType)h=a.linkTrackVars,f=a.linkTrackEvents,a.pe&&(e=a.pe.substring(0,1).toUpperCase()+a.pe.substring(1),a[e]&&(h=a[e].Kb,f=a[e].Jb));h&&(h=","+h+","+a.G.join(",")+",");f&&h&&(h+=",events,")}b&&(b=","+b+",");for(f=0;f<d.length;f++)e=d[f],(g=a[e])&&(!h||0<=h.indexOf(","+e+","))&&(!b||0<=b.indexOf(","+e+","))&&c(e,g)};a.q=function(c,
b,d,f,e){var g="",h,l,k,q,m=0;"contextData"==c&&(c="c");if(b){for(h in b)if(!(Object.prototype[h]||e&&h.substring(0,e.length)!=e)&&b[h]&&(!d||0<=d.indexOf(","+(f?f+".":"")+h+","))){k=!1;if(m)for(l=0;l<m.length;l++)h.substring(0,m[l].length)==m[l]&&(k=!0);if(!k&&(""==g&&(g+="&"+c+"."),l=b[h],e&&(h=h.substring(e.length)),0<h.length))if(k=h.indexOf("."),0<k)l=h.substring(0,k),k=(e?e:"")+l+".",m||(m=[]),m.push(k),g+=a.q(l,b,d,f,k);else if("boolean"==typeof l&&(l=l?"true":"false"),l){if("retrieveLightData"==
f&&0>e.indexOf(".contextData."))switch(k=h.substring(0,4),q=h.substring(4),h){case "transactionID":h="xact";break;case "channel":h="ch";break;case "campaign":h="v0";break;default:a.Ga(q)&&("prop"==k?h="c"+q:"eVar"==k?h="v"+q:"list"==k?h="l"+q:"hier"==k&&(h="h"+q,l=l.substring(0,255)))}g+="&"+a.escape(h)+"="+a.escape(l)}}""!=g&&(g+="&."+c)}return g};a.usePostbacks=0;a.wb=function(){var c="",b,d,f,e,g,h,l,k,q="",m="",n=e="";if(a.lightProfileID)b=a.P,(q=a.lightTrackVars)&&(q=","+q+","+a.ga.join(",")+
",");else{b=a.g;if(a.pe||a.linkType)q=a.linkTrackVars,m=a.linkTrackEvents,a.pe&&(e=a.pe.substring(0,1).toUpperCase()+a.pe.substring(1),a[e]&&(q=a[e].Kb,m=a[e].Jb));q&&(q=","+q+","+a.G.join(",")+",");m&&(m=","+m+",",q&&(q+=",events,"));a.events2&&(n+=(""!=n?",":"")+a.events2)}if(a.visitor&&a.visitor.getCustomerIDs){e=p;if(g=a.visitor.getCustomerIDs())for(d in g)Object.prototype[d]||(f=g[d],"object"==typeof f&&(e||(e={}),f.id&&(e[d+".id"]=f.id),f.authState&&(e[d+".as"]=f.authState)));e&&(c+=a.q("cid",
e))}a.AudienceManagement&&a.AudienceManagement.isReady()&&(c+=a.q("d",a.AudienceManagement.getEventCallConfigParams()));for(d=0;d<b.length;d++){e=b[d];g=a[e];f=e.substring(0,4);h=e.substring(4);g||("events"==e&&n?(g=n,n=""):"marketingCloudOrgID"==e&&a.visitor&&(g=a.visitor.marketingCloudOrgID));if(g&&(!q||0<=q.indexOf(","+e+","))){switch(e){case "customerPerspective":e="cp";break;case "marketingCloudOrgID":e="mcorgid";break;case "supplementalDataID":e="sdid";break;case "timestamp":e="ts";break;case "dynamicVariablePrefix":e=
"D";break;case "visitorID":e="vid";break;case "marketingCloudVisitorID":e="mid";break;case "analyticsVisitorID":e="aid";break;case "audienceManagerLocationHint":e="aamlh";break;case "audienceManagerBlob":e="aamb";break;case "authState":e="as";break;case "pageURL":e="g";255<g.length&&(a.pageURLRest=g.substring(255),g=g.substring(0,255));break;case "pageURLRest":e="-g";break;case "referrer":e="r";break;case "vmk":case "visitorMigrationKey":e="vmt";break;case "visitorMigrationServer":e="vmf";a.ssl&&
a.visitorMigrationServerSecure&&(g="");break;case "visitorMigrationServerSecure":e="vmf";!a.ssl&&a.visitorMigrationServer&&(g="");break;case "charSet":e="ce";break;case "visitorNamespace":e="ns";break;case "cookieDomainPeriods":e="cdp";break;case "cookieLifetime":e="cl";break;case "variableProvider":e="vvp";break;case "currencyCode":e="cc";break;case "channel":e="ch";break;case "transactionID":e="xact";break;case "campaign":e="v0";break;case "latitude":e="lat";break;case "longitude":e="lon";break;
case "resolution":e="s";break;case "colorDepth":e="c";break;case "javascriptVersion":e="j";break;case "javaEnabled":e="v";break;case "cookiesEnabled":e="k";break;case "browserWidth":e="bw";break;case "browserHeight":e="bh";break;case "connectionType":e="ct";break;case "homepage":e="hp";break;case "events":n&&(g+=(""!=g?",":"")+n);if(m)for(h=g.split(","),g="",f=0;f<h.length;f++)l=h[f],k=l.indexOf("="),0<=k&&(l=l.substring(0,k)),k=l.indexOf(":"),0<=k&&(l=l.substring(0,k)),0<=m.indexOf(","+l+",")&&(g+=
(g?",":"")+h[f]);break;case "events2":g="";break;case "contextData":c+=a.q("c",a[e],q,e);g="";break;case "lightProfileID":e="mtp";break;case "lightStoreForSeconds":e="mtss";a.lightProfileID||(g="");break;case "lightIncrementBy":e="mti";a.lightProfileID||(g="");break;case "retrieveLightProfiles":e="mtsr";break;case "deleteLightProfiles":e="mtsd";break;case "retrieveLightData":a.retrieveLightProfiles&&(c+=a.q("mts",a[e],q,e));g="";break;default:a.Ga(h)&&("prop"==f?e="c"+h:"eVar"==f?e="v"+h:"list"==
f?e="l"+h:"hier"==f&&(e="h"+h,g=g.substring(0,255)))}g&&(c+="&"+e+"="+("pev"!=e.substring(0,3)?a.escape(g):g))}"pev3"==e&&a.e&&(c+=a.e)}a.fa&&(c+="&lrt="+a.fa,a.fa=null);return c};a.C=function(a){var b=a.tagName;if("undefined"!=""+a.Pb||"undefined"!=""+a.Fb&&"HTML"!=(""+a.Fb).toUpperCase())return"";b=b&&b.toUpperCase?b.toUpperCase():"";"SHAPE"==b&&(b="");b&&(("INPUT"==b||"BUTTON"==b)&&a.type&&a.type.toUpperCase?b=a.type.toUpperCase():!b&&a.href&&(b="A"));return b};a.Ca=function(a){var b=k.location,
d=a.href?a.href:"",f,e,g;f=d.indexOf(":");e=d.indexOf("?");g=d.indexOf("/");d&&(0>f||0<=e&&f>e||0<=g&&f>g)&&(e=a.protocol&&1<a.protocol.length?a.protocol:b.protocol?b.protocol:"",f=b.pathname.lastIndexOf("/"),d=(e?e+"//":"")+(a.host?a.host:b.host?b.host:"")+("/"!=d.substring(0,1)?b.pathname.substring(0,0>f?0:f)+"/":"")+d);return d};a.M=function(c){var b=a.C(c),d,f,e="",g=0;return b&&(d=c.protocol,f=c.onclick,!c.href||"A"!=b&&"AREA"!=b||f&&d&&!(0>d.toLowerCase().indexOf("javascript"))?f?(e=a.replace(a.replace(a.replace(a.replace(""+
f,"\r",""),"\n",""),"\t","")," ",""),g=2):"INPUT"==b||"SUBMIT"==b?(c.value?e=c.value:c.innerText?e=c.innerText:c.textContent&&(e=c.textContent),g=3):"IMAGE"==b&&c.src&&(e=c.src):e=a.Ca(c),e)?{id:e.substring(0,100),type:g}:0};a.Nb=function(c){for(var b=a.C(c),d=a.M(c);c&&!d&&"BODY"!=b;)if(c=c.parentElement?c.parentElement:c.parentNode)b=a.C(c),d=a.M(c);d&&"BODY"!=b||(c=0);c&&(b=c.onclick?""+c.onclick:"",0<=b.indexOf(".tl(")||0<=b.indexOf(".trackLink("))&&(c=0);return c};a.Eb=function(){var c,b,d=a.linkObject,
f=a.linkType,e=a.linkURL,g,h;a.ha=1;d||(a.ha=0,d=a.clickObject);if(d){c=a.C(d);for(b=a.M(d);d&&!b&&"BODY"!=c;)if(d=d.parentElement?d.parentElement:d.parentNode)c=a.C(d),b=a.M(d);b&&"BODY"!=c||(d=0);if(d&&!a.linkObject){var l=d.onclick?""+d.onclick:"";if(0<=l.indexOf(".tl(")||0<=l.indexOf(".trackLink("))d=0}}else a.ha=1;!e&&d&&(e=a.Ca(d));e&&!a.linkLeaveQueryString&&(g=e.indexOf("?"),0<=g&&(e=e.substring(0,g)));if(!f&&e){var m=0,q=0,n;if(a.trackDownloadLinks&&a.linkDownloadFileTypes)for(l=e.toLowerCase(),
g=l.indexOf("?"),h=l.indexOf("#"),0<=g?0<=h&&h<g&&(g=h):g=h,0<=g&&(l=l.substring(0,g)),g=a.linkDownloadFileTypes.toLowerCase().split(","),h=0;h<g.length;h++)(n=g[h])&&l.substring(l.length-(n.length+1))=="."+n&&(f="d");if(a.trackExternalLinks&&!f&&(l=e.toLowerCase(),a.Fa(l)&&(a.linkInternalFilters||(a.linkInternalFilters=k.location.hostname),g=0,a.linkExternalFilters?(g=a.linkExternalFilters.toLowerCase().split(","),m=1):a.linkInternalFilters&&(g=a.linkInternalFilters.toLowerCase().split(",")),g))){for(h=
0;h<g.length;h++)n=g[h],0<=l.indexOf(n)&&(q=1);q?m&&(f="e"):m||(f="e")}}a.linkObject=d;a.linkURL=e;a.linkType=f;if(a.trackClickMap||a.trackInlineStats)a.e="",d&&(f=a.pageName,e=1,d=d.sourceIndex,f||(f=a.pageURL,e=0),k.s_objectID&&(b.id=k.s_objectID,d=b.type=1),f&&b&&b.id&&c&&(a.e="&pid="+a.escape(f.substring(0,255))+(e?"&pidt="+e:"")+"&oid="+a.escape(b.id.substring(0,100))+(b.type?"&oidt="+b.type:"")+"&ot="+c+(d?"&oi="+d:"")))};a.xb=function(){var c=a.ha,b=a.linkType,d=a.linkURL,f=a.linkName;b&&(d||
f)&&(b=b.toLowerCase(),"d"!=b&&"e"!=b&&(b="o"),a.pe="lnk_"+b,a.pev1=d?a.escape(d):"",a.pev2=f?a.escape(f):"",c=1);a.abort&&(c=0);if(a.trackClickMap||a.trackInlineStats||a.Ab()){var b={},d=0,e=a.cookieRead("s_sq"),g=e?e.split("&"):0,h,l,k,e=0;if(g)for(h=0;h<g.length;h++)l=g[h].split("="),f=a.unescape(l[0]).split(","),l=a.unescape(l[1]),b[l]=f;f=a.account.split(",");h={};for(k in a.contextData)k&&!Object.prototype[k]&&"a.activitymap."==k.substring(0,14)&&(h[k]=a.contextData[k],a.contextData[k]="");
a.e=a.q("c",h)+(a.e?a.e:"");if(c||a.e){c&&!a.e&&(e=1);for(l in b)if(!Object.prototype[l])for(k=0;k<f.length;k++)for(e&&(g=b[l].join(","),g==a.account&&(a.e+=("&"!=l.charAt(0)?"&":"")+l,b[l]=[],d=1)),h=0;h<b[l].length;h++)g=b[l][h],g==f[k]&&(e&&(a.e+="&u="+a.escape(g)+("&"!=l.charAt(0)?"&":"")+l+"&u=0"),b[l].splice(h,1),d=1);c||(d=1);if(d){e="";h=2;!c&&a.e&&(e=a.escape(f.join(","))+"="+a.escape(a.e),h=1);for(l in b)!Object.prototype[l]&&0<h&&0<b[l].length&&(e+=(e?"&":"")+a.escape(b[l].join(","))+"="+
a.escape(l),h--);a.cookieWrite("s_sq",e)}}}return c};a.yb=function(){if(!a.Ib){var c=new Date,b=n.location,d,f,e=f=d="",g="",h="",l="1.2",k=a.cookieWrite("s_cc","true",0)?"Y":"N",m="",p="";if(c.setUTCDate&&(l="1.3",(0).toPrecision&&(l="1.5",c=[],c.forEach))){l="1.6";f=0;d={};try{f=new Iterator(d),f.next&&(l="1.7",c.reduce&&(l="1.8",l.trim&&(l="1.8.1",Date.parse&&(l="1.8.2",Object.create&&(l="1.8.5")))))}catch(r){}}d=screen.width+"x"+screen.height;e=navigator.javaEnabled()?"Y":"N";f=screen.pixelDepth?
screen.pixelDepth:screen.colorDepth;g=a.w.innerWidth?a.w.innerWidth:a.d.documentElement.offsetWidth;h=a.w.innerHeight?a.w.innerHeight:a.d.documentElement.offsetHeight;try{a.b.addBehavior("#default#homePage"),m=a.b.Ob(b)?"Y":"N"}catch(s){}try{a.b.addBehavior("#default#clientCaps"),p=a.b.connectionType}catch(t){}a.resolution=d;a.colorDepth=f;a.javascriptVersion=l;a.javaEnabled=e;a.cookiesEnabled=k;a.browserWidth=g;a.browserHeight=h;a.connectionType=p;a.homepage=m;a.Ib=1}};a.Q={};a.loadModule=function(c,
b){var d=a.Q[c];if(!d){d=k["AppMeasurement_Module_"+c]?new k["AppMeasurement_Module_"+c](a):{};a.Q[c]=a[c]=d;d.Za=function(){return d.eb};d.gb=function(b){if(d.eb=b)a[c+"_onLoad"]=b,a.ba(c+"_onLoad",[a,d],1)||b(a,d)};try{Object.defineProperty?Object.defineProperty(d,"onLoad",{get:d.Za,set:d.gb}):d._olc=1}catch(f){d._olc=1}}b&&(a[c+"_onLoad"]=b,a.ba(c+"_onLoad",[a,d],1)||b(a,d))};a.o=function(c){var b,d;for(b in a.Q)if(!Object.prototype[b]&&(d=a.Q[b])&&(d._olc&&d.onLoad&&(d._olc=0,d.onLoad(a,d)),d[c]&&
d[c]()))return 1;return 0};a.Ab=function(){return a.ActivityMap&&a.ActivityMap._c?!0:!1};a.Bb=function(){var c=Math.floor(1E13*Math.random()),b=a.visitorSampling,d=a.visitorSamplingGroup,d="s_vsn_"+(a.visitorNamespace?a.visitorNamespace:a.account)+(d?"_"+d:""),f=a.cookieRead(d);if(b){b*=100;f&&(f=parseInt(f));if(!f){if(!a.cookieWrite(d,c))return 0;f=c}if(f%1E4>b)return 0}return 1};a.R=function(c,b){var d,f,e,g,h,l;for(d=0;2>d;d++)for(f=0<d?a.ua:a.g,e=0;e<f.length;e++)if(g=f[e],(h=c[g])||c["!"+g]){if(!b&&
("contextData"==g||"retrieveLightData"==g)&&a[g])for(l in a[g])h[l]||(h[l]=a[g][l]);a[g]=h}};a.Qa=function(c,b){var d,f,e,g;for(d=0;2>d;d++)for(f=0<d?a.ua:a.g,e=0;e<f.length;e++)g=f[e],c[g]=a[g],b||c[g]||(c["!"+g]=1)};a.sb=function(a){var b,d,f,e,g,h=0,l,k="",m="";if(a&&255<a.length&&(b=""+a,d=b.indexOf("?"),0<d&&(l=b.substring(d+1),b=b.substring(0,d),e=b.toLowerCase(),f=0,"http://"==e.substring(0,7)?f+=7:"https://"==e.substring(0,8)&&(f+=8),d=e.indexOf("/",f),0<d&&(e=e.substring(f,d),g=b.substring(d),
b=b.substring(0,d),0<=e.indexOf("google")?h=",q,ie,start,search_key,word,kw,cd,":0<=e.indexOf("yahoo.co")&&(h=",p,ei,"),h&&l)))){if((a=l.split("&"))&&1<a.length){for(f=0;f<a.length;f++)e=a[f],d=e.indexOf("="),0<d&&0<=h.indexOf(","+e.substring(0,d)+",")?k+=(k?"&":"")+e:m+=(m?"&":"")+e;k&&m?l=k+"&"+m:m=""}d=253-(l.length-m.length)-b.length;a=b+(0<d?g.substring(0,d):"")+"?"+l}return a};a.Ta=function(c){var b=a.d.visibilityState,d=["webkitvisibilitychange","visibilitychange"];b||(b=a.d.webkitVisibilityState);
if(b&&"prerender"==b){if(c)for(b=0;b<d.length;b++)a.d.addEventListener(d[b],function(){var b=a.d.visibilityState;b||(b=a.d.webkitVisibilityState);"visible"==b&&c()});return!1}return!0};a.X=!1;a.J=!1;a.ib=function(){a.J=!0;a.H()};a.Y=!1;a.S=!1;a.jb=function(c){a.marketingCloudVisitorID=c.MCMID;a.visitorOptedOut=c.MCOPTOUT;a.analyticsVisitorID=c.MCAID;a.audienceManagerLocationHint=c.MCAAMLH;a.audienceManagerBlob=c.MCAAMB;a.S=!0;a.H()};a.Sa=function(c){a.maxDelay||(a.maxDelay=250);return a.o("_d")?(c&&
setTimeout(function(){c()},a.maxDelay),!1):!0};a.W=!1;a.I=!1;a.qa=function(){a.I=!0;a.H()};a.isReadyToTrack=function(){var c=!0,b=a.visitor;a.X||a.J||(a.Ta(a.ib)?a.J=!0:a.X=!0);if(a.X&&!a.J)return!1;b&&b.isAllowed()&&(a.Y||a.marketingCloudVisitorID||!b.getVisitorValues||(a.Y=!0,a.marketingCloudVisitorID?a.S=!0:b.getVisitorValues(a.jb)),c=!a.Y||a.S||a.marketingCloudVisitorID?!0:!1);a.W||a.I||(a.Sa(a.qa)?a.I=!0:a.W=!0);a.W&&!a.I&&(c=!1);return c};a.l=p;a.r=0;a.callbackWhenReadyToTrack=function(c,b,
d){var f;f={};f.nb=c;f.mb=b;f.kb=d;a.l==p&&(a.l=[]);a.l.push(f);0==a.r&&(a.r=setInterval(a.H,100))};a.H=function(){var c;if(a.isReadyToTrack()&&(a.hb(),a.l!=p))for(;0<a.l.length;)c=a.l.shift(),c.mb.apply(c.nb,c.kb)};a.hb=function(){a.r&&(clearInterval(a.r),a.r=0)};a.ab=function(c){var b,d,f=p,e=p;if(!a.isReadyToTrack()){b=[];if(c!=p)for(d in f={},c)f[d]=c[d];e={};a.Qa(e,!0);b.push(f);b.push(e);a.callbackWhenReadyToTrack(a,a.track,b);return!0}return!1};a.ub=function(){var c=a.cookieRead("s_fid"),b=
"",d="",f;f=8;var e=4;if(!c||0>c.indexOf("-")){for(c=0;16>c;c++)f=Math.floor(Math.random()*f),b+="0123456789ABCDEF".substring(f,f+1),f=Math.floor(Math.random()*e),d+="0123456789ABCDEF".substring(f,f+1),f=e=16;c=b+"-"+d}a.cookieWrite("s_fid",c,1)||(c=0);return c};a.t=a.track=function(c,b){var d,f=new Date,e="s"+Math.floor(f.getTime()/108E5)%10+Math.floor(1E13*Math.random()),g=f.getYear(),g="t="+a.escape(f.getDate()+"/"+f.getMonth()+"/"+(1900>g?g+1900:g)+" "+f.getHours()+":"+f.getMinutes()+":"+f.getSeconds()+
" "+f.getDay()+" "+f.getTimezoneOffset());a.visitor&&a.visitor.getAuthState&&(a.authState=a.visitor.getAuthState());a.o("_s");a.ab(c)||(b&&a.R(b),c&&(d={},a.Qa(d,0),a.R(c)),a.Bb()&&!a.visitorOptedOut&&(a.analyticsVisitorID||a.marketingCloudVisitorID||(a.fid=a.ub()),a.Eb(),a.usePlugins&&a.doPlugins&&a.doPlugins(a),a.account&&(a.abort||(a.trackOffline&&!a.timestamp&&(a.timestamp=Math.floor(f.getTime()/1E3)),f=k.location,a.pageURL||(a.pageURL=f.href?f.href:f),a.referrer||a.Ra||(f=a.Util.getQueryParam("adobe_mc_ref",
null,null,!0),a.referrer=f||void 0===f?void 0===f?"":f:n.document.referrer),a.Ra=1,a.referrer=a.sb(a.referrer),a.o("_g")),a.xb()&&!a.abort&&(a.visitor&&!a.supplementalDataID&&a.visitor.getSupplementalDataID&&(a.supplementalDataID=a.visitor.getSupplementalDataID("AppMeasurement:"+a._in,a.expectSupplementalData?!1:!0)),a.yb(),g+=a.wb(),a.cb(e,g),a.o("_t"),a.referrer=""))),c&&a.R(d,1));a.abort=a.supplementalDataID=a.timestamp=a.pageURLRest=a.linkObject=a.clickObject=a.linkURL=a.linkName=a.linkType=k.s_objectID=
a.pe=a.pev1=a.pev2=a.pev3=a.e=a.lightProfileID=0};a.ta=[];a.registerPreTrackCallback=function(c){for(var b=[],d=1;d<arguments.length;d++)b.push(arguments[d]);"function"==typeof c?a.ta.push([c,b]):a.debugTracking&&a.D("DEBUG: Non function type passed to registerPreTrackCallback")};a.Wa=function(c){a.oa(a.ta,c)};a.ra=[];a.registerPostTrackCallback=function(c){for(var b=[],d=1;d<arguments.length;d++)b.push(arguments[d]);"function"==typeof c?a.ra.push([c,b]):a.debugTracking&&a.D("DEBUG: Non function type passed to registerPostTrackCallback")};
a.Va=function(c){a.oa(a.ra,c)};a.oa=function(c,b){if("object"==typeof c)for(var d=0;d<c.length;d++){var f=c[d][0],e=c[d][1].slice();e.unshift(b);if("function"==typeof f)try{f.apply(null,e)}catch(g){a.debugTracking&&a.D(g.message)}}};a.tl=a.trackLink=function(c,b,d,f,e){a.linkObject=c;a.linkType=b;a.linkName=d;e&&(a.k=c,a.v=e);return a.track(f)};a.trackLight=function(c,b,d,f){a.lightProfileID=c;a.lightStoreForSeconds=b;a.lightIncrementBy=d;return a.track(f)};a.clearVars=function(){var c,b;for(c=0;c<
a.g.length;c++)if(b=a.g[c],"prop"==b.substring(0,4)||"eVar"==b.substring(0,4)||"hier"==b.substring(0,4)||"list"==b.substring(0,4)||"channel"==b||"events"==b||"eventList"==b||"products"==b||"productList"==b||"purchaseID"==b||"transactionID"==b||"state"==b||"zip"==b||"campaign"==b)a[b]=void 0};a.tagContainerMarker="";a.cb=function(c,b){var d=a.Xa()+"/"+c+"?AQB=1&ndh=1&pf=1&"+(a.pa()?"callback=s_c_il["+a._in+"].doPostbacks&et=1&":"")+b+"&AQE=1";a.Wa(d);a.Ua(d);a.T()};a.Xa=function(){var c=a.Ya();return"http"+
(a.ssl?"s":"")+"://"+c+"/b/ss/"+a.account+"/"+(a.mobile?"5.":"")+(a.pa()?"10":"1")+"/JS-"+a.version+(a.Hb?"T":"")+(a.tagContainerMarker?"-"+a.tagContainerMarker:"")};a.pa=function(){return a.AudienceManagement&&a.AudienceManagement.isReady()||0!=a.usePostbacks};a.Ya=function(){var c=a.dc,b=a.trackingServer;b?a.trackingServerSecure&&a.ssl&&(b=a.trackingServerSecure):(c=c?(""+c).toLowerCase():"d1","d1"==c?c="112":"d2"==c&&(c="122"),b=a.$a()+"."+c+".2o7.net");return b};a.$a=function(){var c=a.visitorNamespace;
c||(c=a.account.split(",")[0],c=c.replace(/[^0-9a-z]/gi,""));return c};a.Pa=/{(%?)(.*?)(%?)}/;a.Lb=RegExp(a.Pa.source,"g");a.rb=function(c){if("object"==typeof c.dests)for(var b=0;b<c.dests.length;++b){var d=c.dests[b];if("string"==typeof d.c&&"aa."==d.id.substr(0,3))for(var f=d.c.match(a.Lb),e=0;e<f.length;++e){var g=f[e],h=g.match(a.Pa),k="";"%"==h[1]&&"timezone_offset"==h[2]?k=(new Date).getTimezoneOffset():"%"==h[1]&&"timestampz"==h[2]&&(k=a.vb());d.c=d.c.replace(g,a.escape(k))}}};a.vb=function(){var c=
new Date,b=new Date(6E4*Math.abs(c.getTimezoneOffset()));return a.j(4,c.getFullYear())+"-"+a.j(2,c.getMonth()+1)+"-"+a.j(2,c.getDate())+"T"+a.j(2,c.getHours())+":"+a.j(2,c.getMinutes())+":"+a.j(2,c.getSeconds())+(0<c.getTimezoneOffset()?"-":"+")+a.j(2,b.getUTCHours())+":"+a.j(2,b.getUTCMinutes())};a.j=function(a,b){return(Array(a+1).join(0)+b).slice(-a)};a.la={};a.doPostbacks=function(c){if("object"==typeof c)if(a.rb(c),"object"==typeof a.AudienceManagement&&"function"==typeof a.AudienceManagement.isReady&&
a.AudienceManagement.isReady()&&"function"==typeof a.AudienceManagement.passData)a.AudienceManagement.passData(c);else if("object"==typeof c&&"object"==typeof c.dests)for(var b=0;b<c.dests.length;++b){var d=c.dests[b];"object"==typeof d&&"string"==typeof d.c&&"string"==typeof d.id&&"aa."==d.id.substr(0,3)&&(a.la[d.id]=new Image,a.la[d.id].alt="",a.la[d.id].src=d.c)}};a.Ua=function(c){a.i||a.zb();a.i.push(c);a.ea=a.B();a.Na()};a.zb=function(){a.i=a.Cb();a.i||(a.i=[])};a.Cb=function(){var c,b;if(a.ka()){try{(b=
k.localStorage.getItem(a.ia()))&&(c=k.JSON.parse(b))}catch(d){}return c}};a.ka=function(){var c=!0;a.trackOffline&&a.offlineFilename&&k.localStorage&&k.JSON||(c=!1);return c};a.Da=function(){var c=0;a.i&&(c=a.i.length);a.p&&c++;return c};a.T=function(){if(a.p&&(a.A&&a.A.complete&&a.A.F&&a.A.na(),a.p))return;a.Ea=p;if(a.ja)a.ea>a.O&&a.La(a.i),a.ma(500);else{var c=a.lb();if(0<c)a.ma(c);else if(c=a.Aa())a.p=1,a.Db(c),a.Gb(c)}};a.ma=function(c){a.Ea||(c||(c=0),a.Ea=setTimeout(a.T,c))};a.lb=function(){var c;
if(!a.trackOffline||0>=a.offlineThrottleDelay)return 0;c=a.B()-a.Ja;return a.offlineThrottleDelay<c?0:a.offlineThrottleDelay-c};a.Aa=function(){if(0<a.i.length)return a.i.shift()};a.Db=function(c){if(a.debugTracking){var b="AppMeasurement Debug: "+c;c=c.split("&");var d;for(d=0;d<c.length;d++)b+="\n\t"+a.unescape(c[d]);a.D(b)}};a.bb=function(){return a.marketingCloudVisitorID||a.analyticsVisitorID};a.V=!1;var t;try{t=JSON.parse('{"x":"y"}')}catch(w){t=null}t&&"y"==t.x?(a.V=!0,a.U=function(a){return JSON.parse(a)}):
k.$&&k.$.parseJSON?(a.U=function(a){return k.$.parseJSON(a)},a.V=!0):a.U=function(){return null};a.Gb=function(c){var b,d,f;a.bb()&&2047<c.length&&("undefined"!=typeof XMLHttpRequest&&(b=new XMLHttpRequest,"withCredentials"in b?d=1:b=0),b||"undefined"==typeof XDomainRequest||(b=new XDomainRequest,d=2),b&&(a.AudienceManagement&&a.AudienceManagement.isReady()||0!=a.usePostbacks)&&(a.V?b.va=!0:b=0));!b&&a.Oa&&(c=c.substring(0,2047));!b&&a.d.createElement&&(0!=a.usePostbacks||a.AudienceManagement&&a.AudienceManagement.isReady())&&
(b=a.d.createElement("SCRIPT"))&&"async"in b&&((f=(f=a.d.getElementsByTagName("HEAD"))&&f[0]?f[0]:a.d.body)?(b.type="text/javascript",b.setAttribute("async","async"),d=3):b=0);b||(b=new Image,b.alt="",b.abort||"undefined"===typeof k.InstallTrigger||(b.abort=function(){b.src=p}));b.Ka=Date.now();b.xa=function(){try{b.F&&(clearTimeout(b.F),b.F=0)}catch(a){}};b.onload=b.na=function(){b.Ka&&(a.fa=Date.now()-b.Ka);a.Va(c);b.xa();a.pb();a.Z();a.p=0;a.T();if(b.va){b.va=!1;try{a.doPostbacks(a.U(b.responseText))}catch(d){}}};
b.onabort=b.onerror=b.Ba=function(){b.xa();(a.trackOffline||a.ja)&&a.p&&a.i.unshift(a.ob);a.p=0;a.ea>a.O&&a.La(a.i);a.Z();a.ma(500)};b.onreadystatechange=function(){4==b.readyState&&(200==b.status?b.na():b.Ba())};a.Ja=a.B();if(1==d||2==d){var e=c.indexOf("?");f=c.substring(0,e);e=c.substring(e+1);e=e.replace(/&callback=[a-zA-Z0-9_.\[\]]+/,"");1==d?(b.open("POST",f,!0),b.send(e)):2==d&&(b.open("POST",f),b.send(e))}else if(b.src=c,3==d){if(a.Ha)try{f.removeChild(a.Ha)}catch(g){}f.firstChild?f.insertBefore(b,
f.firstChild):f.appendChild(b);a.Ha=a.A}b.F=setTimeout(function(){b.F&&(b.complete?b.na():(a.trackOffline&&b.abort&&b.abort(),b.Ba()))},5E3);a.ob=c;a.A=k["s_i_"+a.replace(a.account,",","_")]=b;if(a.useForcedLinkTracking&&a.K||a.v)a.forcedLinkTrackingTimeout||(a.forcedLinkTrackingTimeout=250),a.aa=setTimeout(a.Z,a.forcedLinkTrackingTimeout)};a.pb=function(){if(a.ka()&&!(a.Ia>a.O))try{k.localStorage.removeItem(a.ia()),a.Ia=a.B()}catch(c){}};a.La=function(c){if(a.ka()){a.Na();try{k.localStorage.setItem(a.ia(),
k.JSON.stringify(c)),a.O=a.B()}catch(b){}}};a.Na=function(){if(a.trackOffline){if(!a.offlineLimit||0>=a.offlineLimit)a.offlineLimit=10;for(;a.i.length>a.offlineLimit;)a.Aa()}};a.forceOffline=function(){a.ja=!0};a.forceOnline=function(){a.ja=!1};a.ia=function(){return a.offlineFilename+"-"+a.visitorNamespace+a.account};a.B=function(){return(new Date).getTime()};a.Fa=function(a){a=a.toLowerCase();return 0!=a.indexOf("#")&&0!=a.indexOf("about:")&&0!=a.indexOf("opera:")&&0!=a.indexOf("javascript:")?!0:
!1};a.setTagContainer=function(c){var b,d,f;a.Hb=c;for(b=0;b<a._il.length;b++)if((d=a._il[b])&&"s_l"==d._c&&d.tagContainerName==c){a.R(d);if(d.lmq)for(b=0;b<d.lmq.length;b++)f=d.lmq[b],a.loadModule(f.n);if(d.ml)for(f in d.ml)if(a[f])for(b in c=a[f],f=d.ml[f],f)!Object.prototype[b]&&("function"!=typeof f[b]||0>(""+f[b]).indexOf("s_c_il"))&&(c[b]=f[b]);if(d.mmq)for(b=0;b<d.mmq.length;b++)f=d.mmq[b],a[f.m]&&(c=a[f.m],c[f.f]&&"function"==typeof c[f.f]&&(f.a?c[f.f].apply(c,f.a):c[f.f].apply(c)));if(d.tq)for(b=
0;b<d.tq.length;b++)a.track(d.tq[b]);d.s=a;break}};a.Util={urlEncode:a.escape,urlDecode:a.unescape,cookieRead:a.cookieRead,cookieWrite:a.cookieWrite,getQueryParam:function(c,b,d,f){var e,g="";b||(b=a.pageURL?a.pageURL:k.location);d=d?d:"&";if(!c||!b)return g;b=""+b;e=b.indexOf("?");if(0>e)return g;b=d+b.substring(e+1)+d;if(!f||!(0<=b.indexOf(d+c+d)||0<=b.indexOf(d+c+"="+d))){e=b.indexOf("#");0<=e&&(b=b.substr(0,e)+d);e=b.indexOf(d+c+"=");if(0>e)return g;b=b.substring(e+d.length+c.length+1);e=b.indexOf(d);
0<=e&&(b=b.substring(0,e));0<b.length&&(g=a.unescape(b));return g}},getIeVersion:function(){if(document.documentMode)return document.documentMode;for(var a=7;4<a;a--){var b=document.createElement("div");b.innerHTML="\x3c!--[if IE "+a+"]><span></span><![endif]--\x3e";if(b.getElementsByTagName("span").length)return a}return null}};a.G="supplementalDataID timestamp dynamicVariablePrefix visitorID marketingCloudVisitorID analyticsVisitorID audienceManagerLocationHint authState fid vmk visitorMigrationKey visitorMigrationServer visitorMigrationServerSecure charSet visitorNamespace cookieDomainPeriods fpCookieDomainPeriods cookieLifetime pageName pageURL customerPerspective referrer contextData currencyCode lightProfileID lightStoreForSeconds lightIncrementBy retrieveLightProfiles deleteLightProfiles retrieveLightData".split(" ");
a.g=a.G.concat("purchaseID variableProvider channel server pageType transactionID campaign state zip events events2 products audienceManagerBlob tnt".split(" "));a.ga="timestamp charSet visitorNamespace cookieDomainPeriods cookieLifetime contextData lightProfileID lightStoreForSeconds lightIncrementBy".split(" ");a.P=a.ga.slice(0);a.ua="account allAccounts debugTracking visitor visitorOptedOut trackOffline offlineLimit offlineThrottleDelay offlineFilename usePlugins doPlugins configURL visitorSampling visitorSamplingGroup linkObject clickObject linkURL linkName linkType trackDownloadLinks trackExternalLinks trackClickMap trackInlineStats linkLeaveQueryString linkTrackVars linkTrackEvents linkDownloadFileTypes linkExternalFilters linkInternalFilters useForcedLinkTracking forcedLinkTrackingTimeout trackingServer trackingServerSecure ssl abort mobile dc lightTrackVars maxDelay expectSupplementalData usePostbacks registerPreTrackCallback registerPostTrackCallback AudienceManagement".split(" ");
for(m=0;250>=m;m++)76>m&&(a.g.push("prop"+m),a.P.push("prop"+m)),a.g.push("eVar"+m),a.P.push("eVar"+m),6>m&&a.g.push("hier"+m),4>m&&a.g.push("list"+m);m="pe pev1 pev2 pev3 latitude longitude resolution colorDepth javascriptVersion javaEnabled cookiesEnabled browserWidth browserHeight connectionType homepage pageURLRest marketingCloudOrgID".split(" ");a.g=a.g.concat(m);a.G=a.G.concat(m);a.ssl=0<=k.location.protocol.toLowerCase().indexOf("https");a.charSet="UTF-8";a.contextData={};a.offlineThrottleDelay=
0;a.offlineFilename="AppMeasurement.offline";a.Ja=0;a.ea=0;a.O=0;a.Ia=0;a.linkDownloadFileTypes="exe,zip,wav,mp3,mov,mpg,avi,wmv,pdf,doc,docx,xls,xlsx,ppt,pptx";a.w=k;a.d=k.document;try{if(a.Oa=!1,navigator){var v=navigator.userAgent;if("Microsoft Internet Explorer"==navigator.appName||0<=v.indexOf("MSIE ")||0<=v.indexOf("Trident/")&&0<=v.indexOf("Windows NT 6"))a.Oa=!0}}catch(x){}a.Z=function(){a.aa&&(k.clearTimeout(a.aa),a.aa=p);a.k&&a.K&&a.k.dispatchEvent(a.K);a.v&&("function"==typeof a.v?a.v():
a.k&&a.k.href&&(a.d.location=a.k.href));a.k=a.K=a.v=0};a.Ma=function(){a.b=a.d.body;a.b?(a.u=function(c){var b,d,f,e,g;if(!(a.d&&a.d.getElementById("cppXYctnr")||c&&c["s_fe_"+a._in])){if(a.wa)if(a.useForcedLinkTracking)a.b.removeEventListener("click",a.u,!1);else{a.b.removeEventListener("click",a.u,!0);a.wa=a.useForcedLinkTracking=0;return}else a.useForcedLinkTracking=0;a.clickObject=c.srcElement?c.srcElement:c.target;try{if(!a.clickObject||a.N&&a.N==a.clickObject||!(a.clickObject.tagName||a.clickObject.parentElement||
a.clickObject.parentNode))a.clickObject=0;else{var h=a.N=a.clickObject;a.da&&(clearTimeout(a.da),a.da=0);a.da=setTimeout(function(){a.N==h&&(a.N=0)},1E4);f=a.Da();a.track();if(f<a.Da()&&a.useForcedLinkTracking&&c.target){for(e=c.target;e&&e!=a.b&&"A"!=e.tagName.toUpperCase()&&"AREA"!=e.tagName.toUpperCase();)e=e.parentNode;if(e&&(g=e.href,a.Fa(g)||(g=0),d=e.target,c.target.dispatchEvent&&g&&(!d||"_self"==d||"_top"==d||"_parent"==d||k.name&&d==k.name))){try{b=a.d.createEvent("MouseEvents")}catch(l){b=
new k.MouseEvent}if(b){try{b.initMouseEvent("click",c.bubbles,c.cancelable,c.view,c.detail,c.screenX,c.screenY,c.clientX,c.clientY,c.ctrlKey,c.altKey,c.shiftKey,c.metaKey,c.button,c.relatedTarget)}catch(m){b=0}b&&(b["s_fe_"+a._in]=b.s_fe=1,c.stopPropagation(),c.stopImmediatePropagation&&c.stopImmediatePropagation(),c.preventDefault(),a.k=c.target,a.K=b)}}}}}catch(n){a.clickObject=0}}},a.b&&a.b.attachEvent?a.b.attachEvent("onclick",a.u):a.b&&a.b.addEventListener&&(navigator&&(0<=navigator.userAgent.indexOf("WebKit")&&
a.d.createEvent||0<=navigator.userAgent.indexOf("Firefox/2")&&k.MouseEvent)&&(a.wa=1,a.useForcedLinkTracking=1,a.b.addEventListener("click",a.u,!0)),a.b.addEventListener("click",a.u,!1))):setTimeout(a.Ma,30)};a.qb();a.Qb||(r?a.setAccount(r):a.D("Error, missing Report Suite ID in AppMeasurement initialization"),a.Ma(),a.loadModule("ActivityMap"))}
function s_gi(r){var a,k=window.s_c_il,p,n,m=r.split(","),s,u,t=0;if(k)for(p=0;!t&&p<k.length;){a=k[p];if("s_c"==a._c&&(a.account||a.oun))if(a.account&&a.account==r)t=1;else for(n=a.account?a.account:a.oun,n=a.allAccounts?a.allAccounts:n.split(","),s=0;s<m.length;s++)for(u=0;u<n.length;u++)m[s]==n[u]&&(t=1);p++}t?a.setAccount&&a.setAccount(r):a=new AppMeasurement(r);return a}AppMeasurement.getInstance=s_gi;window.s_objectID||(window.s_objectID=0);
function s_pgicq(){var r=window,a=r.s_giq,k,p,n;if(a)for(k=0;k<a.length;k++)p=a[k],n=s_gi(p.oun),n.setAccount(p.un),n.setTagContainer(p.tagContainerName);r.s_giq=0}s_pgicq();
window.document.cookie = "s_sq=; max-age=0; domain=.shopping.yahoo.co.jp;path=/";
