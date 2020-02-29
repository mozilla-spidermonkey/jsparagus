(function() {
    var _w = [];
    _w.APD = ["microsoft.com", "accdn.lpsnmedia.net", "amp.azure.net", "assets.onestore.ms", "bat.bing.com", "bid.g.doubleclick.net", "c.conversionlogic.net", "cdnssl.clicktale.net", "connect.facebook.net", "d.impactradius-event.com", "fpt.microsoft.com", "googleads.g.doubleclick.net", "login.live.com", "lpcdn.lpsnmedia.net", "lptag.liveperson.net", "mem.gfx.ms", "mscom.demdex.net", "nexus.ensighten.com", "pnapi.invoca.net", "px.owneriq.net", "sales.liveperson.net", "static-assets.dev.fs.liveperson.com", "statics-storeexp-eus-ms-com.akamaized.net", "statics-storeexp-wcus-ms-com.akamaized.net", "storage.live.com", "www.googleadservices.com", "va.v.liveperson.net", "web.vortex.data.microsoft.com", "www.bing.com"], _w.APC = ["c-", "f-", "m-", "vsj-", "pdp-", "mfp-", "pi-", "lp", "chatContainer", "EstimatedDelivery", "transcript_scroll"],
    function msstore1() {
        var d = document.createElement("A"),
            m = _w.APD ? _w.APD.slice() : [],
            l = _w.APC ? _w.APC.slice() : [];

        function p(e, t, n) {
            for (var o = 0; o < t.length; o++)
                if ("C" === n && 0 === e.toUpperCase().indexOf(t[o].toUpperCase()) || "S" === n && 0 <= e.toUpperCase().indexOf(t[o].toUpperCase())) return !0;
            return !1
        }

        function f(e, t, n) {
            var o, a, s, i;
            e.parentNode && 1 === e.parentNode.nodeType && 1 === e.nodeType && e.offsetWidth && 20 < e.offsetWidth && (o = t += "D", a = e.tagName, s = "/store/api/GetBucket?Kind=R&Verb=" + o + "&Tag=" + a + "&Info=" + n, (i = new XMLHttpRequest).onreadystatechange = function () { }, i.open("GET", s, !0), i.send())
        }

        function e(e) {
            (window.MutationObserver || window.WebKitMutationObserver) && ("undefined" != typeof MutationObserver && new MutationObserver(function (e) {
                if (e)
                    for (var t = 0; t < e.length; t++) {
                        var n = e[t].addedNodes;
                        if (n && n.length)
                            for (var o = 0; o < n.length; o++) {
                                var a = n[o];
                                switch (a.tagName) {
                                    case "IFRAME":
                                    case "IMG":
                                    case "SCRIPT":
                                    case "LINK":
                                        (r = a).src && (d.href = r.src, window.location.hostname.indexOf(d.hostname) < 0 && !p(d.hostname, m, "S") && f(r, "RS", d.hostname));
                                        break;
                                    case "DIV":
                                    case "LI":
                                        c = i = void 0, i = (s = a).className, (c = i && i.trim()) && 0 !== c.indexOf("bing_") && !p(c, l, "C") && f(s, "RC", i);
                                        break;
                                    case "OBJECT":
                                        a.type && 0 <= a.type.indexOf("flash") && f(a, "RN", a.outerHTML.substr(0, 100));
                                        break;
                                    case "CENTER":
                                        f(a, "RN", a.outerHTML.substr(0, 100));
                                        break;
                                    default:
                                        return
                                }
                            }
                    }
                var s, i, c, r
            }).observe(e, {
                childList: !0,
                subtree: !0
            }))
        }
        e(document.getElementsByTagName("head")[0]);
        window.addEventListener("load", function () {
            e(document.getElementsByTagName("body")[0])
        })
    }();

    !function msstore2() {
        var n = document.createElement("A"),
            o = _w.APD ? _w.APD.slice() : [];

        function r(e, t, n) {
            var o = "/store/api/GetBucket?Kind=P&Verb=" + e + "&Tag=" + t + "&Info=" + n,
                r = new XMLHttpRequest;
            r.onreadystatechange = function () { }, r.open("GET", o, !0), r.send()
        }

        function i(e) {
            switch (e.tagName) {
                case "IFRAME":
                case "IMG":
                case "SCRIPT":
                case "LINK":
                    return (t = e).src && (n.href = t.src, window.location.hostname.indexOf(n.hostname) < 0 && ! function (e, t) {
                        for (var n = 0; n < t.length; n++)
                            if (0 <= e.toUpperCase().indexOf(t[n].toUpperCase())) return !0;
                        return !1
                    }(n.hostname, o) && r("D", t.tagName, n.hostname)), !0;
                case "OBJECT":
                    return e.type && 0 <= e.type.indexOf("flash") && r("D", e.tagName, e.outerHTML), !0;
                case "CENTER":
                    return r("D", e.tagName, e.outerHTML), !0;
                default:
                    return !0
            }
            var t
        }

        function e(t) {
            return function (e) {
                return i(e) ? t.apply(this, arguments) : document.createElement(e.tagName)
            }
        }

        function t(e) {
            Object.freeze && Object.freeze(e)
        }
        document.instWrite = document.write, document.write = function (e) {
            return r("D", "DW", e), document.instWrite(e)
        }, document.instWriteln = document.writeln, document.writeln = function (e) {
            return r("D", "DWL", e), document.instWriteln(e)
        }, window.instAlert = window.alert, window.alert = function (e) {
            return r("D", "WA", e), window.instAlert(e)
        }, window.instConfirm = window.confirm, window.confirm = function (e) {
            return r("D", "WC", e), window.instConfirm(e)
        }, window.instModalDiagolue = window.showModalDialog, window.showModalDialog = function (e) {
            return r("D", "WSMD", e), window.instModalDiagolue(e)
        }, window.instModalLessDiagolue = window.showModelessDialog, window.showModelessDialog = function (e) {
            return r("D", "WSMLD", e), window.instModalLessDiagolue(e)
        }, "undefined" != typeof Element && Element.prototype && (Element.prototype.appendChild = e(Element.prototype.appendChild), Element.prototype.insertBefore = e(Element.prototype.insertBefore), Element.prototype.replaceChild = e(Element.prototype.replaceChild), t(Element.prototype)), document.appendChild = e(document.appendChild), document.insertBefore = e(document.insertBefore), document.replaceChild = e(document.replaceChild), "undefined" != typeof HTMLDocument && HTMLDocument.prototype && (HTMLDocument.prototype.appendChild = e(HTMLDocument.prototype.appendChild), HTMLDocument.prototype.insertBefore = e(HTMLDocument.prototype.insertBefore), HTMLDocument.prototype.replaceChild = e(HTMLDocument.prototype.replaceChild), t(HTMLDocument.prototype))
    }();
} )();
