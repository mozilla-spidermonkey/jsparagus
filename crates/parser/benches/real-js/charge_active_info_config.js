requirejs.config( {
  baseUrl: '../../../themes/activity/cat_default/js',
  paths: {
    apps: 'apps'
  }
} );

requirejs( ['apps/charge_active_info'], function ( charge ) {
  charge.init();
} );