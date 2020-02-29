//DIL library ver 7.0 starts here
"function" !== typeof window.DIL && (window.DIL = function (a, c) {
    var e = [], d, h; a !== Object(a) && (a = {}); var g, k, m, p, n, t, y, w, K, L, E, B, C; g = a.partner; k = a.containerNSID; m = !!a.disableDestinationPublishingIframe; p = a.iframeAkamaiHTTPS; n = a.mappings; t = a.uuidCookie; y = !0 === a.enableErrorReporting; w = a.visitorService; K = a.declaredId; L = !0 === a.delayAllUntilWindowLoad; E = !0 === a.disableIDSyncs; B = "undefined" === typeof a.secureDataCollection || !0 === a.secureDataCollection; C = "boolean" === typeof a.isCoopSafe ? a.isCoopSafe : null; var M, I, F,
        N, O; M = !0 === a.disableDefaultRequest; I = a.afterResultForDefaultRequest; F = a.dpIframeSrc; N = a.visitorConstructor; O = !0 === a.disableCORS; y && DIL.errorModule.activate(); y = !0 === window._dil_unit_tests; (d = c) && e.push(d + ""); if (!g || "string" !== typeof g) return d = "DIL partner is invalid or not specified in initConfig", DIL.errorModule.handleError({ name: "error", message: d, filename: "dil.js" }), Error(d); d = "DIL containerNSID is invalid or not specified in initConfig, setting to default of 0"; if (k || "number" === typeof k) k = parseInt(k,
            10), !isNaN(k) && 0 <= k && (d = ""); d && (k = 0, e.push(d), d = ""); h = DIL.getDil(g, k); if (h instanceof DIL && h.api.getPartner() === g && h.api.getContainerNSID() === k) return h; if (this instanceof DIL) DIL.registerDil(this, g, k); else return new DIL(a, "DIL was not instantiated with the 'new' operator, returning a valid instance with partner = " + g + " and containerNSID = " + k); var x = {
                IS_HTTPS: B || "https:" === document.location.protocol, MILLIS_PER_DAY: 864E5, DIL_COOKIE_NAME: "AAMC_" + encodeURIComponent(g) + "_" + k, FIRST_PARTY_SYNCS: "AMSYNCS",
                FIRST_PARTY_SYNCS_ON_PAGE: "AMSYNCSOP", REGION: "REGION", SIX_MONTHS_IN_MINUTES: 259200, IE_VERSION: function () { if (document.documentMode) return document.documentMode; for (var b = 7; 4 < b; b--) { var l = document.createElement("div"); l.innerHTML = "\x3c!--[if IE " + b + "]><span></span><![endif]--\x3e"; if (l.getElementsByTagName("span").length) return b } return null }()
            }; x.IS_IE_LESS_THAN_10 = "number" === typeof x.IE_VERSION && 10 > x.IE_VERSION; var J = { stuffed: {} }, u = {}, q = {
                firingQueue: [], fired: [], firing: !1, sent: [], errored: [], reservedKeys: {
                    sids: !0,
                    pdata: !0, logdata: !0, callback: !0, postCallbackFn: !0, useImageRequest: !0
                }, firstRequestHasFired: !1, abortRequests: !1, num_of_cors_responses: 0, num_of_cors_errors: 0, corsErrorSources: [], num_of_img_responses: 0, num_of_img_errors: 0, platformParams: { d_nsid: k + "", d_rtbd: "json", d_jsonv: DIL.jsonVersion + "", d_dst: "1" }, nonModStatsParams: { d_rtbd: !0, d_dst: !0, d_cts: !0, d_rs: !0 }, modStatsParams: null, adms: {
                    TIME_TO_CATCH_ALL_REQUESTS_RELEASE: 2E3, calledBack: !1, mid: null, noVisitorAPI: !1, VisitorAPI: null, instance: null, releaseType: "no VisitorAPI",
                    isOptedOut: !0, isOptedOutCallbackCalled: !1, admsProcessingStarted: !1, process: function (b) {
                        try {
                            if (!this.admsProcessingStarted) {
                            this.admsProcessingStarted = !0; var l = this, z, f, a, c; if ("function" === typeof b && "function" === typeof b.getInstance) {
                                if (w === Object(w) && (z = w.namespace) && "string" === typeof z) f = b.getInstance(z, { idSyncContainerID: k }); else { this.releaseType = "no namespace"; this.releaseRequests(); return } if (f === Object(f) && f instanceof b && "function" === typeof f.isAllowed && "function" === typeof f.getMarketingCloudVisitorID &&
                                    "function" === typeof f.getCustomerIDs && "function" === typeof f.isOptedOut) {
                                    this.VisitorAPI = b; if (!f.isAllowed()) { this.releaseType = "VisitorAPI not allowed"; this.releaseRequests(); return } this.instance = f; a = function (b) { "VisitorAPI" !== l.releaseType && (l.mid = b, l.releaseType = "VisitorAPI", l.releaseRequests()) }; c = f.getMarketingCloudVisitorID(a); if ("string" === typeof c && c.length) { a(c); return } setTimeout(function () { "VisitorAPI" !== l.releaseType && (l.releaseType = "timeout", l.releaseRequests()) }, this.getLoadTimeout());
                                    return
                                } this.releaseType = "invalid instance"
                            } else this.noVisitorAPI = !0; this.releaseRequests()
                            }
                        } catch (d) { this.releaseRequests() }
                    }, releaseRequests: function () { this.calledBack = !0; q.registerRequest() }, getMarketingCloudVisitorID: function () { return this.instance ? this.instance.getMarketingCloudVisitorID() : null }, getMIDQueryString: function () { var b = v.isPopulatedString, l = this.getMarketingCloudVisitorID(); b(this.mid) && this.mid === l || (this.mid = l); return b(this.mid) ? "d_mid=" + this.mid + "&" : "" }, getCustomerIDs: function () {
                        return this.instance ?
                            this.instance.getCustomerIDs() : null
                    }, getCustomerIDsQueryString: function (b) { if (b === Object(b)) { var l = "", z = [], f = [], a, c; for (a in b) b.hasOwnProperty(a) && (f[0] = a, c = b[a], c === Object(c) && (f[1] = c.id || "", f[2] = c.authState || 0, z.push(f), f = [])); if (f = z.length) for (b = 0; b < f; b++)l += "&d_cid_ic=" + r.encodeAndBuildRequest(z[b], "%01"); return l } return "" }, getIsOptedOut: function () {
                        this.instance ? this.instance.isOptedOut([this, this.isOptedOutCallback], this.VisitorAPI.OptOut.GLOBAL, !0) : (this.isOptedOut = !1, this.isOptedOutCallbackCalled =
                            !0)
                    }, isOptedOutCallback: function (b) { this.isOptedOut = b; this.isOptedOutCallbackCalled = !0; q.registerRequest() }, getLoadTimeout: function () { var b = this.instance; if (b) { if ("function" === typeof b.getLoadTimeout) return b.getLoadTimeout(); if ("undefined" !== typeof b.loadTimeout) return b.loadTimeout } return this.TIME_TO_CATCH_ALL_REQUESTS_RELEASE }
                }, declaredId: {
                    declaredId: { init: null, request: null }, declaredIdCombos: {}, setDeclaredId: function (b, l) {
                        var z = v.isPopulatedString, f = encodeURIComponent; if (b === Object(b) && z(l)) {
                            var a =
                                b.dpid, c = b.dpuuid, d = null; if (z(a) && z(c)) { d = f(a) + "$" + f(c); if (!0 === this.declaredIdCombos[d]) return "setDeclaredId: combo exists for type '" + l + "'"; this.declaredIdCombos[d] = !0; this.declaredId[l] = { dpid: a, dpuuid: c }; return "setDeclaredId: succeeded for type '" + l + "'" }
                        } return "setDeclaredId: failed for type '" + l + "'"
                    }, getDeclaredIdQueryString: function () {
                        var b = this.declaredId.request, l = this.declaredId.init, a = encodeURIComponent, f = ""; null !== b ? f = "&d_dpid=" + a(b.dpid) + "&d_dpuuid=" + a(b.dpuuid) : null !== l && (f = "&d_dpid=" +
                            a(l.dpid) + "&d_dpuuid=" + a(l.dpuuid)); return f
                    }
                }, registerRequest: function (b) {
                    var l = this.firingQueue; b === Object(b) && l.push(b); this.firing || !l.length || L && !DIL.windowLoaded || (this.adms.isOptedOutCallbackCalled || this.adms.getIsOptedOut(), this.adms.calledBack && !this.adms.isOptedOut && this.adms.isOptedOutCallbackCalled && (this.adms.isOptedOutCallbackCalled = !1, b = l.shift(), b.src = b.src.replace(/demdex.net\/event\?d_nsid=/, "demdex.net/event?" + this.adms.getMIDQueryString() + "d_nsid="), v.isPopulatedString(b.corsPostData) &&
                        (b.corsPostData = b.corsPostData.replace(/^d_nsid=/, this.adms.getMIDQueryString() + "d_nsid=")), D.fireRequest(b), this.firstRequestHasFired || "script" !== b.tag && "cors" !== b.tag || (this.firstRequestHasFired = !0)))
                }, processVisitorAPI: function () { this.adms.process(N || window.Visitor) }, getCoopQueryString: function () { var b = ""; !0 === C ? b = "&d_coop_safe=1" : !1 === C && (b = "&d_coop_unsafe=1"); return b }
            }; B = function () {
                var b = "http://fast.", l = "?d_nsid=" + k + "#" + encodeURIComponent(document.location.href); if ("string" === typeof F && F.length) return F +
                    l; x.IS_HTTPS && (b = !0 === p ? "https://fast." : "https://"); return b + g + ".demdex.net/dest5.html" + l
            }; var A = {
                MAX_SYNCS_LENGTH: 649, id: "destination_publishing_iframe_" + g + "_" + k, url: B(), onPagePixels: [], iframeHost: null, getIframeHost: function (b) { if ("string" === typeof b) { var l = b.split("/"); if (3 <= l.length) return l[0] + "//" + l[2]; e.push("getIframeHost: url is malformed: " + b); return b } }, iframe: null, iframeHasLoaded: !1, sendingMessages: !1, messages: [], messagesPosted: [], messagesReceived: [], ibsDeleted: [], jsonForComparison: [],
                jsonDuplicates: [], jsonWaiting: [], jsonProcessed: [], canSetThirdPartyCookies: !0, receivedThirdPartyCookiesNotification: !1, newIframeCreated: null, iframeIdChanged: !1, originalIframeHasLoadedAlready: null, regionChanged: !1, timesRegionChanged: 0, attachIframe: function () {
                    function b() {
                        f = document.createElement("iframe"); f.sandbox = "allow-scripts allow-same-origin"; f.title = "Adobe ID Syncing iFrame"; f.id = a.id; f.name = a.id + "_name"; f.style.cssText = "display: none; width: 0; height: 0;"; f.src = a.url; a.newIframeCreated = !0;
                        l(); document.body.appendChild(f)
                    } function l() { f.addEventListener("load", function () { f.className = "aamIframeLoaded"; a.iframeHasLoaded = !0; a.requestToProcess() }) } if (!x.IS_IE_LESS_THAN_10) {
                        var a = this, f = document.getElementById(this.id); f ? "IFRAME" !== f.nodeName ? (this.id += "_2", this.iframeIdChanged = !0, b()) : (this.newIframeCreated = !1, "aamIframeLoaded" !== f.className ? (this.originalIframeHasLoadedAlready = !1, l()) : (this.iframeHasLoaded = this.originalIframeHasLoadedAlready = !0, this.iframe = f, this.requestToProcess())) :
                            b(); this.iframe = f
                    }
                }, requestToProcess: function (b, l) {
                    function a() { f.jsonForComparison.push(b); f.jsonWaiting.push([b, l]) } var f = this, c, d; c = q.adms.instance; b === Object(b) && c === Object(c) && c.idSyncContainerID === k && (A.ibsDeleted.push(b.ibs), delete b.ibs); if (b && !v.isEmptyObject(b)) if (c = JSON.stringify(b.ibs || []), d = JSON.stringify(b.dests || []), this.jsonForComparison.length) {
                        var e = !1, g, h, n; g = 0; for (h = this.jsonForComparison.length; g < h; g++)if (n = this.jsonForComparison[g], c === JSON.stringify(n.ibs || []) && d === JSON.stringify(n.dests ||
                            [])) { e = !0; break } e ? this.jsonDuplicates.push(b) : a()
                    } else a(); this.receivedThirdPartyCookiesNotification && this.jsonWaiting.length && (c = this.jsonWaiting.shift(), !1 === this.newIframeCreated && delete c[0].ibs, this.process(c[0], c[1]), this.requestToProcess()); this.iframeHasLoaded && this.messages.length && !this.sendingMessages && (this.sendingMessages = !0, this.sendMessages())
                }, checkIfRegionChanged: function (b) {
                    var l = r.getDilCookieField(x.REGION); null !== l && "undefined" !== typeof b.dcs_region && parseInt(l, 10) !== b.dcs_region &&
                        (this.regionChanged = !0, this.timesRegionChanged++ , r.setDilCookieField(x.FIRST_PARTY_SYNCS_ON_PAGE, ""), r.setDilCookieField(x.FIRST_PARTY_SYNCS, "")); "undefined" !== typeof b.dcs_region && r.setDilCookieField(x.REGION, b.dcs_region)
                }, processSyncOnPage: function (b) { var l, a, f; if ((l = b.ibs) && l instanceof Array && (a = l.length)) for (b = 0; b < a; b++)f = l[b], f.syncOnPage && this.checkFirstPartyCookie(f, "", "syncOnPage") }, process: function (b, l) {
                    var a = encodeURIComponent, f, c, d, e, g, h; l === Object(l) && (h = r.encodeAndBuildRequest(["",
                        l.dpid || "", l.dpuuid || ""], ",")); if ((f = b.dests) && f instanceof Array && (c = f.length)) for (d = 0; d < c; d++)e = f[d], g = [a("dests"), a(e.id || ""), a(e.y || ""), a(e.c || "")], this.addMessage(g.join("|")); if ((f = b.ibs) && f instanceof Array && (c = f.length)) for (d = 0; d < c; d++)e = f[d], g = [a("ibs"), a(e.id || ""), a(e.tag || ""), r.encodeAndBuildRequest(e.url || [], ","), a(e.ttl || ""), "", h, e.fireURLSync ? "true" : "false"], e.syncOnPage || (this.canSetThirdPartyCookies ? this.addMessage(g.join("|")) : e.fireURLSync && this.checkFirstPartyCookie(e, g.join("|")));
                    this.jsonProcessed.push(b)
                }, checkFirstPartyCookie: function (b, l, a) { var f = (a = "syncOnPage" === a ? !0 : !1) ? x.FIRST_PARTY_SYNCS_ON_PAGE : x.FIRST_PARTY_SYNCS, c = this.getOnPageSyncData(f), d = !1, e = !1, g = Math.ceil((new Date).getTime() / x.MILLIS_PER_DAY); c ? (c = c.split("*"), e = this.pruneSyncData(c, b.id, g), d = e.dataPresent, e = e.dataValid, d && e || this.fireSync(a, b, l, c, f, g)) : (c = [], this.fireSync(a, b, l, c, f, g)) }, getOnPageSyncData: function (b) {
                    var l = q.adms.instance; return l && "function" === typeof l.idSyncGetOnPageSyncInfo ? l.idSyncGetOnPageSyncInfo() :
                        r.getDilCookieField(b)
                }, pruneSyncData: function (b, l, a) { var f = !1, c = !1, d, e, g; if (b instanceof Array) for (e = 0; e < b.length; e++)d = b[e], g = parseInt(d.split("-")[1], 10), d.match("^" + l + "-") ? (f = !0, a < g ? c = !0 : (b.splice(e, 1), e--)) : a >= g && (b.splice(e, 1), e--); return { dataPresent: f, dataValid: c } }, manageSyncsSize: function (b) { if (b.join("*").length > this.MAX_SYNCS_LENGTH) for (b.sort(function (b, a) { return parseInt(b.split("-")[1], 10) - parseInt(a.split("-")[1], 10) }); b.join("*").length > this.MAX_SYNCS_LENGTH;)b.shift() }, fireSync: function (b,
                    l, a, f, c, d) {
                        function e(b, l, a, f) { return function () { g.onPagePixels[b] = null; var c = g.getOnPageSyncData(a), d = []; if (c) { var c = c.split("*"), e, z, h; e = 0; for (z = c.length; e < z; e++)h = c[e], h.match("^" + l.id + "-") || d.push(h) } g.setSyncTrackingData(d, l, a, f) } } var g = this; if (b) { if ("img" === l.tag) { b = l.url; a = x.IS_HTTPS ? "https:" : "http:"; var h, k, n; f = 0; for (h = b.length; f < h; f++) { k = b[f]; n = /^\/\//.test(k); var w = new Image; w.addEventListener("load", e(this.onPagePixels.length, l, c, d)); w.src = (n ? a : "") + k; this.onPagePixels.push(w) } } } else this.addMessage(a),
                            this.setSyncTrackingData(f, l, c, d)
                }, addMessage: function (b) { this.messages.push(b) }, setSyncTrackingData: function (b, l, a, f) { b.push(l.id + "-" + (f + Math.ceil(l.ttl / 60 / 24))); this.manageSyncsSize(b); r.setDilCookieField(a, b.join("*")) }, sendMessages: function () { var b = "", l = encodeURIComponent; this.regionChanged && (b = l("---destpub-clear-dextp---"), this.regionChanged = !1); this.messages.length && (b = b + l("---destpub-combined---") + this.messages.join("%01"), this.postMessage(b), this.messages = []); this.sendingMessages = !1 }, postMessage: function (b) {
                    DIL.xd.postMessage(b,
                        this.url, this.iframe.contentWindow); this.messagesPosted.push(b)
                }, receiveMessage: function (b) { var l = /^---destpub-to-parent---/; "string" === typeof b && l.test(b) && (l = b.replace(l, "").split("|"), "canSetThirdPartyCookies" === l[0] && (this.canSetThirdPartyCookies = "true" === l[1] ? !0 : !1, this.receivedThirdPartyCookiesNotification = !0, this.requestToProcess()), this.messagesReceived.push(b)) }
            }, G = {
                traits: function (b) { v.isValidPdata(b) && (u.sids instanceof Array || (u.sids = []), r.extendArray(u.sids, b)); return this }, pixels: function (b) {
                    v.isValidPdata(b) &&
                    (u.pdata instanceof Array || (u.pdata = []), r.extendArray(u.pdata, b)); return this
                }, logs: function (b) { v.isValidLogdata(b) && (u.logdata !== Object(u.logdata) && (u.logdata = {}), r.extendObject(u.logdata, b)); return this }, customQueryParams: function (b) { v.isEmptyObject(b) || r.extendObject(u, b, q.reservedKeys); return this }, signals: function (b, l) { var a, f = b; if (!v.isEmptyObject(f)) { if (l && "string" === typeof l) for (a in f = {}, b) b.hasOwnProperty(a) && (f[l + a] = b[a]); r.extendObject(u, f, q.reservedKeys) } return this }, declaredId: function (b) {
                    q.declaredId.setDeclaredId(b,
                        "request"); return this
                }, result: function (b) { "function" === typeof b && (u.callback = b); return this }, afterResult: function (b) { "function" === typeof b && (u.postCallbackFn = b); return this }, useImageRequest: function () { u.useImageRequest = !0; return this }, clearData: function () { u = {}; return this }, submit: function () { D.submitRequest(u); u = {}; return this }, getPartner: function () { return g }, getContainerNSID: function () { return k }, getEventLog: function () { return e }, getState: function () {
                    var b = {}, l = {}; r.extendObject(b, q, { registerRequest: !0 });
                    r.extendObject(l, A, { attachIframe: !0, requestToProcess: !0, process: !0, sendMessages: !0 }); return { initConfig: a, pendingRequest: u, otherRequestInfo: b, destinationPublishingInfo: l }
                }, idSync: function (b) {
                    if (E) return "Error: id syncs have been disabled"; if (b !== Object(b) || "string" !== typeof b.dpid || !b.dpid.length) return "Error: config or config.dpid is empty"; if ("string" !== typeof b.url || !b.url.length) return "Error: config.url is empty"; var l = b.url, a = b.minutesToLive, f = encodeURIComponent, c = A, d, l = l.replace(/^https:/, "").replace(/^http:/,
                        ""); if ("undefined" === typeof a) a = 20160; else if (a = parseInt(a, 10), isNaN(a) || 0 >= a) return "Error: config.minutesToLive needs to be a positive number"; d = r.encodeAndBuildRequest(["", b.dpid, b.dpuuid || ""], ","); b = ["ibs", f(b.dpid), "img", f(l), a, "", d]; c.addMessage(b.join("|")); q.firstRequestHasFired && c.requestToProcess(); return "Successfully queued"
                }, aamIdSync: function (b) {
                    if (E) return "Error: id syncs have been disabled"; if (b !== Object(b) || "string" !== typeof b.dpuuid || !b.dpuuid.length) return "Error: config or config.dpuuid is empty";
                    b.url = "//dpm.demdex.net/ibs:dpid=" + b.dpid + "&dpuuid=" + b.dpuuid; return this.idSync(b)
                }, passData: function (b) { if (v.isEmptyObject(b)) return "Error: json is empty or not an object"; A.ibsDeleted.push(b.ibs); delete b.ibs; D.defaultCallback(b); return b }, getPlatformParams: function () { return q.platformParams }, getEventCallConfigParams: function () {
                    var b = q, a = b.modStatsParams, c = b.platformParams, f; if (!a) {
                        a = {}; for (f in c) c.hasOwnProperty(f) && !b.nonModStatsParams[f] && (a[f.replace(/^d_/, "")] = c[f]); !0 === C ? a.coop_safe = 1 :
                            !1 === C && (a.coop_unsafe = 1); b.modStatsParams = a
                    } return a
                }, setAsCoopSafe: function () { C = !0; return this }, setAsCoopUnsafe: function () { C = !1; return this }
            }, D = {
                corsMetadata: function () {
                    var b = "none", a = !0; "undefined" !== typeof XMLHttpRequest && XMLHttpRequest === Object(XMLHttpRequest) && ("withCredentials" in new XMLHttpRequest ? b = "XMLHttpRequest" : (new Function("/*@cc_on return /^10/.test(@_jscript_version) @*/"))() ? b = "XMLHttpRequest" : "undefined" !== typeof XDomainRequest && XDomainRequest === Object(XDomainRequest) && (a = !1), 0 <
                        Object.prototype.toString.call(window.HTMLElement).indexOf("Constructor") && (a = !1)); return { corsType: b, corsCookiesEnabled: a }
                }(), getCORSInstance: function () { return "none" === this.corsMetadata.corsType ? null : new window[this.corsMetadata.corsType] }, submitRequest: function (b) { q.registerRequest(D.createQueuedRequest(b)); return !0 }, createQueuedRequest: function (b) {
                    var a = b.callback, c = "img", f, d; if (!v.isEmptyObject(n)) {
                        var e; for (f in n) n.hasOwnProperty(f) && (d = n[f], null == d || "" === d || !(f in b) || d in b || d in q.reservedKeys ||
                            (e = b[f], null != e && "" !== e && (b[d] = e)))
                    } v.isValidPdata(b.sids) || (b.sids = []); v.isValidPdata(b.pdata) || (b.pdata = []); v.isValidLogdata(b.logdata) || (b.logdata = {}); b.logdataArray = r.convertObjectToKeyValuePairs(b.logdata, "=", !0); b.logdataArray.push("_ts=" + (new Date).getTime()); "function" !== typeof a && (a = this.defaultCallback); f = this.makeRequestSrcData(b); (d = this.getCORSInstance()) && !0 !== b.useImageRequest && (c = "cors"); return {
                        tag: c, src: f.src, corsSrc: f.corsSrc, callbackFn: a, postCallbackFn: b.postCallbackFn, useImageRequest: !!b.useImageRequest,
                        requestData: b, corsInstance: d, corsPostData: f.corsPostData
                    }
                }, defaultCallback: function (b, a) {
                    A.checkIfRegionChanged(b); A.processSyncOnPage(b); var c, f, d, e, g, h, k, n, w; if ((c = b.stuff) && c instanceof Array && (f = c.length)) for (d = 0; d < f; d++)if ((e = c[d]) && e === Object(e)) {
                        g = e.cn; h = e.cv; k = e.ttl; if ("undefined" === typeof k || "" === k) k = Math.floor(r.getMaxCookieExpiresInMinutes() / 60 / 24); n = e.dmn || "." + document.domain.replace(/^www\./, ""); w = e.type; g && (h || "number" === typeof h) && ("var" !== w && (k = parseInt(k, 10)) && !isNaN(k) && r.setCookie(g,
                            h, 1440 * k, "/", n, !1), J.stuffed[g] = h)
                    } c = b.uuid; v.isPopulatedString(c) && !v.isEmptyObject(t) && (f = t.path, "string" === typeof f && f.length || (f = "/"), d = parseInt(t.days, 10), isNaN(d) && (d = 100), r.setCookie(t.name || "aam_did", c, 1440 * d, f, t.domain || "." + document.domain.replace(/^www\./, ""), !0 === t.secure)); m || q.abortRequests || A.requestToProcess(b, a)
                }, makeRequestSrcData: function (b) {
                b.sids = v.removeEmptyArrayValues(b.sids || []); b.pdata = v.removeEmptyArrayValues(b.pdata || []); var a = q, c = a.platformParams, f = r.encodeAndBuildRequest(b.sids,
                    ","), d = r.encodeAndBuildRequest(b.pdata, ","), e = (b.logdataArray || []).join("&"); delete b.logdataArray; var h = x.IS_HTTPS ? "https://" : "http://", k = a.declaredId.getDeclaredIdQueryString(), n = a.adms.instance ? a.adms.getCustomerIDsQueryString(a.adms.getCustomerIDs()) : "", w = [], m, t, p, y; for (m in b) if (!(m in a.reservedKeys) && b.hasOwnProperty(m)) if (t = b[m], m = encodeURIComponent(m), t instanceof Array) for (p = 0, y = t.length; p < y; p++)w.push(m + "=" + encodeURIComponent(t[p])); else w.push(m + "=" + encodeURIComponent(t)); b = w.length ? "&" +
                        w.join("&") : ""; a = "d_nsid=" + c.d_nsid + a.getCoopQueryString() + k + n + (f.length ? "&d_sid=" + f : "") + (d.length ? "&d_px=" + d : "") + (e.length ? "&d_ld=" + encodeURIComponent(e) : ""); c = "&d_rtbd=" + c.d_rtbd + "&d_jsonv=" + c.d_jsonv + "&d_dst=" + c.d_dst; h = h + g + ".demdex.net/event"; d = f = h + "?" + a + c + b; 2048 < f.length && (f = f.substring(0, 2048).substring(0, f.lastIndexOf("&"))); return { corsSrc: h + "?_ts=" + (new Date).getTime(), src: f, originalSrc: d, corsPostData: a + c + b, isDeclaredIdCall: "" !== k }
                }, fireRequest: function (b) {
                    if ("img" === b.tag) this.fireImage(b);
                    else { var a = q.declaredId, a = a.declaredId.request || a.declaredId.init || {}; this.fireCORS(b, { dpid: a.dpid || "", dpuuid: a.dpuuid || "" }) }
                }, fireImage: function (b) {
                    var a = q, c, f; a.abortRequests || (a.firing = !0, c = new Image(0, 0), a.sent.push(b), c.onload = function () { a.firing = !1; a.fired.push(b); a.num_of_img_responses++; a.registerRequest() }, f = function (c) { d = "imgAbortOrErrorHandler received the event of type " + c.type; e.push(d); a.abortRequests = !0; a.firing = !1; a.errored.push(b); a.num_of_img_errors++; a.registerRequest() }, c.addEventListener("error",
                        f), c.addEventListener("abort", f), c.src = b.src)
                }, fireCORS: function (b, a) {
                    var c = this, f = q, h = this.corsMetadata.corsType, k = b.corsSrc, n = b.corsInstance, w = b.corsPostData, m = b.postCallbackFn, t = "function" === typeof m; if (!f.abortRequests && !O) {
                    f.firing = !0; try {
                        n.open("post", k, !0), "XMLHttpRequest" === h && (n.withCredentials = !0, n.setRequestHeader("Content-Type", "application/x-www-form-urlencoded"), n.onreadystatechange = function () {
                            if (4 === this.readyState && 200 === this.status) a: {
                                var h; try {
                                    if (h = JSON.parse(this.responseText),
                                        h !== Object(h)) { c.handleCORSError(b, a, "Response is not JSON"); break a }
                                } catch (k) { c.handleCORSError(b, a, "Error parsing response as JSON"); break a } E && (A.ibsDeleted.push(h.ibs), delete h.ibs); try { var n = b.callbackFn; f.firing = !1; f.fired.push(b); f.num_of_cors_responses++; n(h, a); t && m(h, a) } catch (k) {
                                k.message = "DIL handleCORSResponse caught error with message " + k.message; d = k.message; e.push(d); k.filename = k.filename || "dil.js"; k.partner = g; DIL.errorModule.handleError(k); try {
                                n({ error: k.name + "|" + k.message }, a), t && m({
                                    error: k.name +
                                    "|" + k.message
                                }, a)
                                } catch (w) { }
                                } finally { f.registerRequest() }
                            }
                        }), n.onerror = function () { c.handleCORSError(b, a, "onerror") }, n.ontimeout = function () { c.handleCORSError(b, a, "ontimeout") }, n.send(w)
                    } catch (p) { this.handleCORSError(b, a, "try-catch") } f.sent.push(b); f.declaredId.declaredId.request = null
                    }
                }, handleCORSError: function (b, a, c) { q.num_of_cors_errors++; q.corsErrorSources.push(c) }, handleRequestError: function (b, a) { var c = q; e.push(b); c.abortRequests = !0; c.firing = !1; c.errored.push(a); c.registerRequest() }
            }, v = {
                isValidPdata: function (b) {
                    return b instanceof
                        Array && this.removeEmptyArrayValues(b).length ? !0 : !1
                }, isValidLogdata: function (b) { return !this.isEmptyObject(b) }, isEmptyObject: function (b) { if (b !== Object(b)) return !0; for (var a in b) if (b.hasOwnProperty(a)) return !1; return !0 }, removeEmptyArrayValues: function (b) { for (var a = 0, c = b.length, f, d = [], a = 0; a < c; a++)f = b[a], "undefined" !== typeof f && null !== f && "" !== f && d.push(f); return d }, isPopulatedString: function (b) { return "string" === typeof b && b.length }
            }, r = {
                convertObjectToKeyValuePairs: function (b, a, c) {
                    var f = [], d, e; a || (a = "=");
                    for (d in b) b.hasOwnProperty(d) && (e = b[d], "undefined" !== typeof e && null !== e && "" !== e && f.push(d + a + (c ? encodeURIComponent(e) : e))); return f
                }, encodeAndBuildRequest: function (b, a) { return b.map(function (b) { return encodeURIComponent(b) }).join(a) }, getCookie: function (b) { b += "="; var a = document.cookie.split(";"), c, f, d; c = 0; for (f = a.length; c < f; c++) { for (d = a[c]; " " === d.charAt(0);)d = d.substring(1, d.length); if (0 === d.indexOf(b)) return decodeURIComponent(d.substring(b.length, d.length)) } return null }, setCookie: function (b, a, c,
                    d, e, g) { var h = new Date; c && (c *= 6E4); document.cookie = b + "=" + encodeURIComponent(a) + (c ? ";expires=" + (new Date(h.getTime() + c)).toUTCString() : "") + (d ? ";path=" + d : "") + (e ? ";domain=" + e : "") + (g ? ";secure" : "") }, extendArray: function (b, a) { return b instanceof Array && a instanceof Array ? (Array.prototype.push.apply(b, a), !0) : !1 }, extendObject: function (b, a, c) { var d; if (b === Object(b) && a === Object(a)) { for (d in a) !a.hasOwnProperty(d) || !v.isEmptyObject(c) && d in c || (b[d] = a[d]); return !0 } return !1 }, getMaxCookieExpiresInMinutes: function () { return x.SIX_MONTHS_IN_MINUTES },
                getCookieField: function (b, a) { var c = this.getCookie(b), d = decodeURIComponent; if ("string" === typeof c) { var c = c.split("|"), e, g; e = 0; for (g = c.length - 1; e < g; e++)if (d(c[e]) === a) return d(c[e + 1]) } return null }, getDilCookieField: function (b) { return this.getCookieField(x.DIL_COOKIE_NAME, b) }, setCookieField: function (b, a, c) {
                    var d = this.getCookie(b), e = !1, g = encodeURIComponent; a = g(a); c = g(c); if ("string" === typeof d) {
                        var d = d.split("|"), h, g = 0; for (h = d.length - 1; g < h; g++)if (d[g] === a) { d[g + 1] = c; e = !0; break } e || (g = d.length, d[g] = a, d[g +
                            1] = c)
                    } else d = [a, c]; this.setCookie(b, d.join("|"), this.getMaxCookieExpiresInMinutes(), "/", this.getDomain(), !1)
                }, setDilCookieField: function (b, a) { return this.setCookieField(x.DIL_COOKIE_NAME, b, a) }, getDomain: function (b) { !b && window.location && (b = window.location.hostname); if (b) if (/^[0-9.]+$/.test(b)) b = ""; else { var a = b.split("."), c = a.length - 1, d = c - 1; 1 < c && 2 >= a[c].length && (2 === a[c - 1].length || 0 > ",DOMAIN_2_CHAR_EXCEPTIONS,".indexOf("," + a[c] + ",")) && d--; if (0 < d) for (b = ""; c >= d;)b = a[c] + (b ? "." : "") + b, c-- } return b }, replaceMethodsWithFunction: function (b,
                    a) { var c; if (b === Object(b) && "function" === typeof a) for (c in b) b.hasOwnProperty(c) && "function" === typeof b[c] && (b[c] = a) }
            }; "error" === g && 0 === k && window.addEventListener("load", function () { DIL.windowLoaded = !0 }); var P = !1, H = function () { P || (P = !0, q.registerRequest(), Q(), m || q.abortRequests || A.attachIframe()) }, Q = function () { m || setTimeout(function () { M || q.firstRequestHasFired || ("function" === typeof I ? G.afterResult(I).submit() : G.submit()) }, DIL.constants.TIME_TO_DEFAULT_REQUEST) }; h = document; "error" !== g && (DIL.windowLoaded ?
                H() : "complete" !== h.readyState && "loaded" !== h.readyState ? window.addEventListener("load", function () { DIL.windowLoaded = !0; H() }) : (DIL.windowLoaded = !0, H())); if ("error" !== g) try { DIL.xd.receiveMessage(function (b) { A.receiveMessage(b.data) }, A.getIframeHost(A.url)) } catch (b) { } q.declaredId.setDeclaredId(K, "init"); q.processVisitorAPI(); x.IS_IE_LESS_THAN_10 && r.replaceMethodsWithFunction(G, function () { return this }); this.api = G; this.getStuffedVariable = function (b) {
                    var a = J.stuffed[b]; a || "number" === typeof a || (a = r.getCookie(b)) ||
                        "number" === typeof a || (a = ""); return a
                }; this.validators = v; this.helpers = r; this.constants = x; this.log = e; y && (this.pendingRequest = u, this.requestController = q, this.setDestinationPublishingUrl = B, this.destinationPublishing = A, this.requestProcs = D, this.variables = J, this.callWindowLoadFunctions = H)
}, DIL.extendStaticPropertiesAndMethods = function (a) { var c; if (a === Object(a)) for (c in a) a.hasOwnProperty(c) && (this[c] = a[c]) }, DIL.extendStaticPropertiesAndMethods({
    version: "7.0", jsonVersion: 1, constants: { TIME_TO_DEFAULT_REQUEST: 50 },
    variables: { scriptNodeList: document.getElementsByTagName("script") }, windowLoaded: !1, dils: {}, isAddedPostWindowLoad: function (a) { this.windowLoaded = "function" === typeof a ? !!a() : "boolean" === typeof a ? a : !0 }, create: function (a) { try { return new DIL(a) } catch (c) { throw Error("Error in attempt to create DIL instance with DIL.create(): " + c.message); } }, registerDil: function (a, c, e) { c = c + "$" + e; c in this.dils || (this.dils[c] = a) }, getDil: function (a, c) {
        var e; "string" !== typeof a && (a = ""); c || (c = 0); e = a + "$" + c; return e in this.dils ?
            this.dils[e] : Error("The DIL instance with partner = " + a + " and containerNSID = " + c + " was not found")
    }, dexGetQSVars: function (a, c, e) { c = this.getDil(c, e); return c instanceof this ? c.getStuffedVariable(a) : "" }, xd: {
        postMessage: function (a, c, e) { c && e.postMessage(a, c.replace(/([^:]+:\/\/[^\/]+).*/, "$1")) }, receiveMessage: function (a, c) {
            var e; try {
            a && (e = function (d) { if ("string" === typeof c && d.origin !== c || "[object Function]" === Object.prototype.toString.call(c) && !1 === c(d.origin)) return !1; a(d) }), window[a ? "addEventListener" :
                "removeEventListener"]("message", e, !1)
            } catch (d) { }
        }
    }
}), DIL.errorModule = function () {
    var a = DIL.create({ partner: "error", containerNSID: 0, disableDestinationPublishingIframe: !0 }), c = { harvestererror: 14138, destpuberror: 14139, dpmerror: 14140, generalerror: 14137, error: 14137, noerrortypedefined: 15021, evalerror: 15016, rangeerror: 15017, referenceerror: 15018, typeerror: 15019, urierror: 15020 }, e = !1; return {
        activate: function () { e = !0 }, handleError: function (d) {
            if (!e) return "DIL error module has not been activated"; d !== Object(d) &&
                (d = {}); var h = d.name ? (d.name + "").toLowerCase() : "", g = []; d = { name: h, filename: d.filename ? d.filename + "" : "", partner: d.partner ? d.partner + "" : "no_partner", site: d.site ? d.site + "" : document.location.href, message: d.message ? d.message + "" : "" }; g.push(h in c ? c[h] : c.noerrortypedefined); a.api.pixels(g).logs(d).useImageRequest().submit(); return "DIL error report sent"
        }, pixelMap: c
    }
}(), DIL.tools = {}, DIL.modules = {
    helpers: {
        handleModuleError: function (a, c, e) {
            var d = ""; c = c || "Error caught in DIL module/submodule: "; a === Object(a) ?
                d = c + (a.message || "err has no message") : (d = c + "err is not a valid object", a = {}); a.message = d; e instanceof DIL && (a.partner = e.api.getPartner()); DIL.errorModule.handleError(a); return this.errorMessage = d
        }
    }
});
DIL.tools.getSearchReferrer = function (a, c) {
    var e = DIL.getDil("error"), d = DIL.tools.decomposeURI(a || document.referrer), h = "", g = "", k = { queryParam: "q" }; return (h = [c === Object(c) ? c : {}, { hostPattern: /aol\./ }, { hostPattern: /ask\./ }, { hostPattern: /bing\./ }, { hostPattern: /google\./ }, { hostPattern: /yahoo\./, queryParam: "p" }].filter(function (a) { return !(!a.hasOwnProperty("hostPattern") || !d.hostname.match(a.hostPattern)) }).shift()) ? {
        valid: !0, name: d.hostname, keywords: (e.helpers.extendObject(k, h), g = k.queryPattern ? (h = ("" + d.search).match(k.queryPattern)) ?
            h[1] : "" : d.uriParams[k.queryParam], decodeURIComponent(g || "").replace(/\+|%20/g, " "))
    } : { valid: !1, name: "", keywords: "" }
};
DIL.tools.decomposeURI = function (a) { var c = document.createElement("a"); c.href = a || document.referrer; return { hash: c.hash, host: c.host.split(":").shift(), hostname: c.hostname, href: c.href, pathname: c.pathname.replace(/^\//, ""), protocol: c.protocol, search: c.search, uriParams: function (a, c) { c.split("&").map(function (c) { c = c.split("="); a[c.shift()] = c.shift() }); return a }({}, c.search.replace(/^(\/|\?)?|\/$/g, "")) } };
DIL.tools.getMetaTags = function () { var a = {}, c = document.getElementsByTagName("meta"), e, d, h, g, k; e = 0; for (h = arguments.length; e < h; e++)if (g = arguments[e], null !== g) for (d = 0; d < c.length; d++)if (k = c[d], k.name === g) { a[g] = k.content; break } return a };
DIL.modules.siteCatalyst = {
    dil: null, handle: DIL.modules.helpers.handleModuleError, init: function (a, c, e, d) {
        try {
            var h = this, g = { name: "DIL Site Catalyst Module Error" }, k = function (a) { g.message = a; DIL.errorModule.handleError(g); return a }; this.options = d === Object(d) ? d : {}; this.dil = null; if (c instanceof DIL) this.dil = c; else return k("dilInstance is not a valid instance of DIL"); g.partner = c.api.getPartner(); if (a !== Object(a)) return k("siteCatalystReportingSuite is not an object"); window.AppMeasurement_Module_DIL = a.m_DIL =
                function (a) {
                    var c = "function" === typeof a.m_i ? a.m_i("DIL") : this; if (c !== Object(c)) return k("m is not an object"); c.trackVars = h.constructTrackVars(e); c.d = 0; c.s = a; c._t = function () {
                        var a, c, d = "," + this.trackVars + ",", e = this.s, g, m = []; g = []; var p = {}, B = !1; if (e !== Object(e)) return k("Error in m._t function: s is not an object"); if (this.d) {
                            if ("function" === typeof e.foreachVar) e.foreachVar(function (a, c) { "undefined" !== typeof c && (p[a] = c, B = !0) }, this.trackVars); else {
                                if (!(e.va_t instanceof Array)) return k("Error in m._t function: s.va_t is not an array");
                                if (e.lightProfileID) (a = e.lightTrackVars) && (a = "," + a + "," + e.vl_mr + ","); else if (e.pe || e.linkType) a = e.linkTrackVars, e.pe && (c = e.pe.substring(0, 1).toUpperCase() + e.pe.substring(1), e[c] && (a = e[c].trackVars)), a && (a = "," + a + "," + e.vl_l + "," + e.vl_l2 + ","); if (a) { c = 0; for (m = a.split(","); c < m.length; c++)0 <= d.indexOf("," + m[c] + ",") && g.push(m[c]); g.length && (d = "," + g.join(",") + ",") } g = 0; for (c = e.va_t.length; g < c; g++)a = e.va_t[g], 0 <= d.indexOf("," + a + ",") && "undefined" !== typeof e[a] && null !== e[a] && "" !== e[a] && (p[a] = e[a], B = !0)
                            } h.includeContextData(e,
                                p).store_populated && (B = !0); B && this.d.api.signals(p, "c_").submit()
                        }
                    }
                }; a.loadModule("DIL"); a.DIL.d = c; return g.message ? g.message : "DIL.modules.siteCatalyst.init() completed with no errors"
        } catch (m) { return this.handle(m, "DIL.modules.siteCatalyst.init() caught error with message ", this.dil) }
    }, constructTrackVars: function (a) {
        var c = [], e, d, h, g, k; if (a === Object(a)) {
            e = a.names; if (e instanceof Array && (h = e.length)) for (d = 0; d < h; d++)g = e[d], "string" === typeof g && g.length && c.push(g); a = a.iteratedNames; if (a instanceof Array &&
                (h = a.length)) for (d = 0; d < h; d++)if (e = a[d], e === Object(e) && (g = e.name, k = parseInt(e.maxIndex, 10), "string" === typeof g && g.length && !isNaN(k) && 0 <= k)) for (e = 0; e <= k; e++)c.push(g + e); if (c.length) return c.join(",")
        } return this.constructTrackVars({ names: "pageName channel campaign products events pe pev1 pev2 pev3".split(" "), iteratedNames: [{ name: "prop", maxIndex: 75 }, { name: "eVar", maxIndex: 250 }] })
    }, includeContextData: function (a, c) {
        var e = {}, d = !1; if (a.contextData === Object(a.contextData)) {
            var h = a.contextData, g = this.options.replaceContextDataPeriodsWith,
            k = this.options.filterFromContextVariables, m = {}, p, n, t, y; "string" === typeof g && g.length || (g = "_"); if (k instanceof Array) for (p = 0, n = k.length; p < n; p++)t = k[p], this.dil.validators.isPopulatedString(t) && (m[t] = !0); for (y in h) !h.hasOwnProperty(y) || m[y] || !(k = h[y]) && "number" !== typeof k || (y = ("contextData." + y).replace(/\./g, g), c[y] = k, d = !0)
        } e.store_populated = d; return e
    }
};
DIL.modules.GA = {
    submitUniversalAnalytics: function (a, c, e) { try { var d = a.getAll()[0], h = d[e || "b"].data.keys; a = {}; var g, k, m, p; g = 0; for (k = h.length; g < k; g++)m = h[g], p = d.get(m), "undefined" === typeof p || "function" === typeof p || p === Object(p) || /^_/.test(m) || /^&/.test(m) || (a[m] = p); c.api.signals(a, "c_").submit(); return a } catch (n) { return "Caught error with message: " + n.message } }, dil: null, arr: null, tv: null, errorMessage: "", defaultTrackVars: ["_setAccount", "_setCustomVar", "_addItem", "_addTrans", "_trackSocial"], defaultTrackVarsObj: null,
    signals: {}, hasSignals: !1, handle: DIL.modules.helpers.handleModuleError, init: function (a, c, e) {
        try {
        this.tv = this.arr = this.dil = null; this.errorMessage = ""; this.signals = {}; this.hasSignals = !1; var d = { name: "DIL GA Module Error" }, h = ""; c instanceof DIL ? (this.dil = c, d.partner = this.dil.api.getPartner()) : (h = "dilInstance is not a valid instance of DIL", d.message = h, DIL.errorModule.handleError(d)); a instanceof Array && a.length ? this.arr = a : (h = "gaArray is not an array or is empty", d.message = h, DIL.errorModule.handleError(d));
            this.tv = this.constructTrackVars(e); this.errorMessage = h
        } catch (g) { this.handle(g, "DIL.modules.GA.init() caught error with message ", this.dil) } finally { return this }
    }, constructTrackVars: function (a) {
        var c = [], e, d, h, g; if (this.defaultTrackVarsObj !== Object(this.defaultTrackVarsObj)) { h = this.defaultTrackVars; g = {}; e = 0; for (d = h.length; e < d; e++)g[h[e]] = !0; this.defaultTrackVarsObj = g } else g = this.defaultTrackVarsObj; if (a === Object(a)) {
            a = a.names; if (a instanceof Array && (d = a.length)) for (e = 0; e < d; e++)h = a[e], "string" === typeof h &&
                h.length && h in g && c.push(h); if (c.length) return c
        } return this.defaultTrackVars
    }, constructGAObj: function (a) { var c = {}; a = a instanceof Array ? a : this.arr; var e, d, h, g; e = 0; for (d = a.length; e < d; e++)h = a[e], h instanceof Array && h.length && (h = [], g = a[e], h instanceof Array && g instanceof Array && Array.prototype.push.apply(h, g), g = h.shift(), "string" === typeof g && g.length && (c[g] instanceof Array || (c[g] = []), c[g].push(h))); return c }, addToSignals: function (a, c) {
        if ("string" !== typeof a || "" === a || null == c || "" === c) return !1; this.signals[a] instanceof
            Array || (this.signals[a] = []); this.signals[a].push(c); return this.hasSignals = !0
    }, constructSignals: function () {
        var a = this.constructGAObj(), c = {
            _setAccount: function (a) { this.addToSignals("c_accountId", a) }, _setCustomVar: function (a, c, d) { "string" === typeof c && c.length && this.addToSignals("c_" + c, d) }, _addItem: function (a, c, d, e, g, h) {
                this.addToSignals("c_itemOrderId", a); this.addToSignals("c_itemSku", c); this.addToSignals("c_itemName", d); this.addToSignals("c_itemCategory", e); this.addToSignals("c_itemPrice", g); this.addToSignals("c_itemQuantity",
                    h)
            }, _addTrans: function (a, c, d, e, g, h, k, m) { this.addToSignals("c_transOrderId", a); this.addToSignals("c_transAffiliation", c); this.addToSignals("c_transTotal", d); this.addToSignals("c_transTax", e); this.addToSignals("c_transShipping", g); this.addToSignals("c_transCity", h); this.addToSignals("c_transState", k); this.addToSignals("c_transCountry", m) }, _trackSocial: function (a, c, d, e) {
                this.addToSignals("c_socialNetwork", a); this.addToSignals("c_socialAction", c); this.addToSignals("c_socialTarget", d); this.addToSignals("c_socialPagePath",
                    e)
            }
        }, e = this.tv, d, h, g, k, m, p; d = 0; for (h = e.length; d < h; d++)if (g = e[d], a.hasOwnProperty(g) && c.hasOwnProperty(g) && (p = a[g], p instanceof Array)) for (k = 0, m = p.length; k < m; k++)c[g].apply(this, p[k])
    }, submit: function () {
        try { if ("" !== this.errorMessage) return this.errorMessage; this.constructSignals(); return this.hasSignals ? (this.dil.api.signals(this.signals).submit(), "Signals sent: " + this.dil.helpers.convertObjectToKeyValuePairs(this.signals, "=", !0) + this.dil.log) : "No signals present" } catch (a) {
            return this.handle(a, "DIL.modules.GA.submit() caught error with message ",
                this.dil)
        }
    }, Stuffer: {
        LIMIT: 5, dil: null, cookieName: null, delimiter: null, errorMessage: "", handle: DIL.modules.helpers.handleModuleError, callback: null, v: function () { return !1 }, init: function (a, c, e) {
            try { this.callback = this.dil = null, this.errorMessage = "", a instanceof DIL ? (this.dil = a, this.v = this.dil.validators.isPopulatedString, this.cookieName = this.v(c) ? c : "aam_ga", this.delimiter = this.v(e) ? e : "|") : this.handle({ message: "dilInstance is not a valid instance of DIL" }, "DIL.modules.GA.Stuffer.init() error: ") } catch (d) {
                this.handle(d,
                    "DIL.modules.GA.Stuffer.init() caught error with message ", this.dil)
            } finally { return this }
        }, process: function (a) {
            var c, e, d, h, g, k; k = !1; var m = 1; if (a === Object(a) && (c = a.stuff) && c instanceof Array && (e = c.length)) for (a = 0; a < e; a++)if ((d = c[a]) && d === Object(d) && (h = d.cn, g = d.cv, h === this.cookieName && this.v(g))) { k = !0; break } if (k) {
                c = g.split(this.delimiter); "undefined" === typeof window._gaq && (window._gaq = []); d = window._gaq; a = 0; for (e = c.length; a < e && !(k = c[a].split("="), g = k[0], k = k[1], this.v(g) && this.v(k) && d.push(["_setCustomVar",
                    m++, g, k, 1]), m > this.LIMIT); a++); this.errorMessage = 1 < m ? "No errors - stuffing successful" : "No valid values to stuff"
            } else this.errorMessage = "Cookie name and value not found in json"; if ("function" === typeof this.callback) return this.callback()
        }, submit: function () {
            try { var a = this; if ("" !== this.errorMessage) return this.errorMessage; this.dil.api.afterResult(function (c) { a.process(c) }).submit(); return "DIL.modules.GA.Stuffer.submit() successful" } catch (c) {
                return this.handle(c, "DIL.modules.GA.Stuffer.submit() caught error with message ",
                    this.dil)
            }
        }
    }
};
//DIL library ver 7.0 ends here

//MSCOMM Instantiation
var mscomDil = DIL.create({
    partner: "mscom",
    containerNSID: 0,
    uuidCookie: {
        name: 'aam_uuid',
        days: 30
    },
    visitorService: {
        namespace: 'EA76ADE95776D2EC7F000101@AdobeOrg'
    }
});

// Audience Manager Cart Flow updates

var jsflat = {
    pids: [],
    skus: [],
    pidsku: [],
    prodnames: []
}, pn = "", pl = [], co = {};

if (typeof _pageBITags == "object") {
    if (typeof _pageBITags.pageTags == "object") {
        pn = _pageBITags.pageTags.pageName;
    }
    if (typeof _pageBITags.pageContracts == "object" && typeof _pageBITags.pageContracts.orderInfo == "object" && _pageBITags.pageContracts.orderInfo.lnItms) {
        pl = _pageBITags.pageContracts.orderInfo.lnItms;
        for (var i = 0; i < pl.length; i++) {
            console.log(pl[i]);
            jsflat.pids.push(pl[i].prdId);
            jsflat.skus.push(pl[i].sku);
            jsflat.pidsku.push(pl[i].prdId + "-" + pl[i].sku);
            jsflat.prodnames.push(pl[i].title);
        }
    } else if (typeof _pageBITags.pageContracts == "object" && typeof _pageBITags.pageContracts.ProductInfo == "object") {
        pl = _pageBITags.pageContracts.ProductInfo;
        jsflat.pids.push(pl.id);
        jsflat.skus.push(pl.sku);
        jsflat.pidsku.push(pl.id + "-" + pl.sku);
        jsflat.prodnames.push(pl.title);
    }
}


if (pn == "receipt" || pn == "congrats") {
    if (jsflat.pids.length != 0) {
        co['c_purchasepid'] = jsflat.pids.join("|");
    }
    if (jsflat.pidsku.length != 0) {
        co['c_purchasepidsku'] = jsflat.pidsku.join("|");
    }
    if (jsflat.prodnames.length != 0) {
        co['c_purchasepnames'] = jsflat.prodnames.join("|");
    }
} else if (~pn.indexOf("interstitial") || ~pn.indexOf("cart")) {
    if (jsflat.pids.length != 0) {
        co['c_cartpid'] = jsflat.pids.join("|");
    }
    if (jsflat.pidsku.length != 0) {
        co['c_cartpidsku'] = jsflat.pidsku.join("|");
    }
    if (jsflat.prodnames.length != 0) {
        co['c_cartpnames'] = jsflat.prodnames.join("|");
    }
} else {
    if (jsflat.pids.length != 0) {
        co['c_viewpid'] = jsflat.pids.join("|");
    }
    if (jsflat.pidsku.length != 0) {
        co['c_viewpidsku'] = jsflat.pidsku.join("|");
    }
    if (jsflat.prodnames.length != 0) {
        co['c_viewpnames'] = jsflat.prodnames.join("|");
    }
}

mscomDil.api.signals(co, "").submit();