var VideoDetailPageEvents;(function(){function l(){if(pMMUtils&&o){var n=!!c.gfbc("mmvc_emb",o);n&&(f>=0&&(s=sb_st(v,f)),e>=0&&(h=sb_st(y,e)));a(n);sj_evt.bind(u,t);sj_be(_w,r,t)}}function t(){sb_ct(s);sb_ct(h);sj_evt.unbind(u,t);sj_ue(_w,r,t)}function a(n){var t={TS:(new Date).getTime(),T:"CI.Show",AppNS:i};n&&(t.InstCategory="VideoSeenInt");pInstr.icd(t)}function v(){var n={TS:(new Date).getTime(),T:"CI.Watch",AppNS:i};pInstr.icd(n)}function y(){var n={TS:(new Date).getTime(),T:"CI.WatchLong",AppNS:i};pInstr.icd(n)}var c=MMUtilsDom,r="unload",u="ajax.unload",n=_w.VDConfig,i=n?n.appns:"video",f=n?n.wst:3e4,e=n?n.wlt:12e4,o=_ge("mmvc"),s,h;l()})(VideoDetailPageEvents||(VideoDetailPageEvents={}))