(function(l) {
    function t(l) {
        return l.replace(/(:|\.)/g, "\\$1")
    }
    var e = "1.4.10",
        o = {
            exclude: [],
            excludeWithin: [],
            offset: 0,
            direction: "top",
            scrollElement: null,
            scrollTarget: null,
            beforeScroll: function() {},
            afterScroll: function() {},
            easing: "swing",
            speed: 400,
            autoCoefficent: 2
        },
        r = function(t) {
            var e = [],
                o = !1,
                r = t.dir && "left" == t.dir ? "scrollLeft" : "scrollTop";
            return this.each(function() {
                if (this != document && this != window) {
                    var t = l(this);
                    t[r]() > 0 ? e.push(this) : (t[r](1), o = t[r]() > 0, o && e.push(this), t[r](0))
                }
            }), e.length || this.each(function() {
                "BODY" === this.nodeName && (e = [this])
            }), "first" === t.el && e.length > 1 && (e = [e[0]]), e
        };
    l.fn.extend({
        scrollable: function(l) {
            var t = r.call(this, {
                dir: l
            });
            return this.pushStack(t)
        },
        firstScrollable: function(l) {
            var t = r.call(this, {
                el: "first",
                dir: l
            });
            return this.pushStack(t)
        },
        smoothScroll: function(e) {
            e = e || {};
            var o = l.extend({}, l.fn.smoothScroll.defaults, e),
                r = l.smoothScroll.filterPath(location.pathname);
            return this.unbind("click.smoothscroll").bind("click.smoothscroll", function(e) {
                var n = this,
                    s = l(this),
                    c = o.exclude,
                    i = o.excludeWithin,
                    a = 0,
                    f = 0,
                    h = !0,
                    u = {},
                    d = location.hostname === n.hostname || !n.hostname,
                    m = o.scrollTarget || (l.smoothScroll.filterPath(n.pathname) || r) === r,
                    p = t(n.hash);
                if (o.scrollTarget || d && m && p) {
                    for (; h && c.length > a;) s.is(t(c[a++])) && (h = !1);
                    for (; h && i.length > f;) s.closest(i[f++]).length && (h = !1)
                } else h = !1;
                h && (e.preventDefault(), l.extend(u, o, {
                    scrollTarget: o.scrollTarget || p,
                    link: n
                }), l.smoothScroll(u))
            }), this
        }
    }), l.smoothScroll = function(t, e) {
        var o, r, n, s, c = 0,
            i = "offset",
            a = "scrollTop",
            f = {},
            h = {};
        "number" == typeof t ? (o = l.fn.smoothScroll.defaults, n = t) : (o = l.extend({}, l.fn.smoothScroll.defaults, t || {}), o.scrollElement && (i = "position", "static" == o.scrollElement.css("position") && o.scrollElement.css("position", "relative"))), o = l.extend({
            link: null
        }, o), a = "left" == o.direction ? "scrollLeft" : a, o.scrollElement ? (r = o.scrollElement, c = r[a]()) : r = l("html, body").firstScrollable(), o.beforeScroll.call(r, o), n = "number" == typeof t ? t : e || l(o.scrollTarget)[i]() && l(o.scrollTarget)[i]()[o.direction] || 0, f[a] = n + c + o.offset, s = o.speed, "auto" === s && (s = f[a] || r.scrollTop(), s /= o.autoCoefficent), h = {
            duration: s,
            easing: o.easing,
            complete: function() {
                o.afterScroll.call(o.link, o)
            }
        }, o.step && (h.step = o.step), r.length ? r.stop().animate(f, h) : o.afterScroll.call(o.link, o)
    }, l.smoothScroll.version = e, l.smoothScroll.filterPath = function(l) {
        return l.replace(/^\//, "").replace(/(index|default).[a-zA-Z]{3,4}$/, "").replace(/\/$/, "")
    }, l.fn.smoothScroll.defaults = o
})(jQuery);


jQuery.cookie = function(a, b, c) {
    if (arguments.length > 1 && String(b) !== "[object Object]") {
        c = jQuery.extend({}, c);
        if (b === null || b === undefined) {
            c.expires = -1
        }
        if (typeof c.expires === "number") {
            var d = c.expires,
                e = c.expires = new Date;
            e.setDate(e.getDate() + d)
        }
        b = String(b);
        return document.cookie = [encodeURIComponent(a), "=", c.raw ? b : encodeURIComponent(b), c.expires ? "; expires=" + c.expires.toUTCString() : "", c.path ? "; path=" + c.path : "", c.domain ? "; domain=" + c.domain : "", c.secure ? "; secure" : ""].join("")
    }
    c = b || {};
    var f, g = c.raw ? function(a) {
        return a
    } : decodeURIComponent;
    return (f = (new RegExp("(?:^|; )" + encodeURIComponent(a) + "=([^;]*)")).exec(document.cookie)) ? g(f[1]) : null
}

jQuery(document).ready(function($) {

    if (typeof tocplus != 'undefined') {
        $.fn.shrinkTOCWidth = function() {
            $(this).css({
                width: 'auto',
                display: 'table'
            });
            if ($.browser.msie && parseInt($.browser.version) == 7)
                $(this).css('width', '');
        }
        if (tocplus.smooth_scroll == 1) {
            var target = hostname = pathname = qs = hash = null;
            $('body a').click(function(event) {
                hostname = $(this).prop('hostname');
                pathname = $(this).prop('pathname');
                qs = $(this).prop('search');
                hash = $(this).prop('hash');
                if (pathname.length > 0) {
                    if (pathname.charAt(0) != '/') {
                        pathname = '/' + pathname;
                    }
                }
                if ((window.location.hostname == hostname) && (window.location.pathname == pathname) && (window.location.search == qs) && (hash !== '')) {
                    var hash_selector = hash.replace(/([ !"$%&'()*+,.\/:;<=>?@[\]^`{|}~])/g, '\\$1');
                    if ($(hash_selector).length > 0)
                        target = hash;
                    else {
                        anchor = hash;
                        anchor = anchor.replace('#', '');
                        target = 'a[name="' + anchor + '"]';
                        if ($(target).length == 0)
                            target = '';
                    }
                    if (typeof tocplus.smooth_scroll_offset != 'undefined') {
                        offset = -1 * tocplus.smooth_scroll_offset;
                    } else {
                        if ($('#wpadminbar').length > 0) {
                            if ($('#wpadminbar').is(':visible'))
                                offset = -30;
                            else
                                offset = 0;
                        } else
                            offset = 0;
                    }
                    if (target) {
                        $.smoothScroll({
                            scrollTarget: target,
                            offset: offset
                        });
                    }
                }
            });
        }
        if (typeof tocplus.visibility_show != 'undefined') {
            var invert = (typeof tocplus.visibility_hide_by_default != 'undefined') ? true : false;
            if ($.cookie)
                var visibility_text = ($.cookie('tocplus_hidetoc')) ? tocplus.visibility_show : tocplus.visibility_hide;
            else
                var visibility_text = tocplus.visibility_hide;
            if (invert)
                visibility_text = (visibility_text == tocplus.visibility_hide) ? tocplus.visibility_show : tocplus.visibility_hide;
            $('#toc_container p.toc__title').append(' <span class="toc__toggle">[<a href="#">' + visibility_text + '</a>]</span>');
            if (visibility_text == tocplus.visibility_show) {
                $('ul.toc__list').hide();
                $('#toc_container').addClass('contracted').shrinkTOCWidth();
            }

            $('span.toc__toggle a').click(function(event) {
                event.preventDefault();
                switch ($(this).html()) {
                    case $('<div/>').html(tocplus.visibility_hide).text():
                        $(this).html(tocplus.visibility_show);
                        if ($.cookie) {
                            if (invert)
                                $.cookie('tocplus_hidetoc', null, {
                                    path: '/'
                                });
                            else
                                $.cookie('tocplus_hidetoc', '1', {
                                    expires: 30,
                                    path: '/'
                                });
                        }
                        $('ul.toc__list').hide('fast');
                        $('#toc_container').addClass('contracted').shrinkTOCWidth();
                        break;
                    case $('<div/>').html(tocplus.visibility_show).text():
                    default:
                        $(this).html(tocplus.visibility_hide);
                        if ($.cookie) {
                            if (invert)
                                $.cookie('tocplus_hidetoc', '1', {
                                    expires: 30,
                                    path: '/'
                                });
                            else
                                $.cookie('tocplus_hidetoc', null, {
                                    path: '/'
                                });
                        }
                        $('#toc_container').css('width', tocplus.width).removeClass('contracted');
                        $('ul.toc__list').show('fast');
                }
            });

        }
    }

   $('.toc .toc__listLink').hover(
        function(){
            $(this).css('border-bottom-color', $(this).css('color'));
        },
        function(){
             $(this).css('border-bottom-color','transparent');
        }
    );
  $('.toc .toc__listLink').focus(
        function(){
            $(this).css('border-bottom-color', $(this).css('color'));
        }
    );
});