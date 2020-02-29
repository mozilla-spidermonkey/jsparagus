function guv() {
    for (var t = [], e = window.location.href.slice(window.location.href.indexOf("?") + 1).split("&"), n = 0; n < e.length; n++) {
        var i = e[n].indexOf("="),
            o = e[n].substring(0, i),
            d = e[n].substring(i + 1, e[n].length);
        t.push(o), t[o] = d
    }
    return t
}

function getCt() {
    var t = window.location.href,
        e = t.indexOf(-1 === t.indexOf("?ct=") ? "&ct=" : "?ct="),
        n = 4;
    return -1 === e && (e = t.indexOf(-1 === t.indexOf("?ctenc=") ? "&ctenc=" : "?ctenc="), n = 7), -1 === e ? null : 7 === n ? decodeURIComponent(t.slice(e + n)) : t.slice(e + n)
}

function addClickTracker(t, e) {
    var n = document.createElement("a");
    n.setAttribute("href", getCt()), n.setAttribute("target", "_blank"), n.innerHTML = "&nbsp;", n.setAttribute("style", "display:block;width:" + t + "px;height:" + e + "px;position:absolute;top:0;left:0;z-index:10005;text-decoration: none;"), document.body.appendChild(n)
}

function GmdBorder(t, e, n) {
    var i = document.createElement("div");
    i.setAttribute("style", "width:" + (e - 2) + "px;height:" + (n - 2) + "px;position:absolute;top:0;left:0;z-index:10000;border:1px solid " + t + ";"), document.body.appendChild(i)
}