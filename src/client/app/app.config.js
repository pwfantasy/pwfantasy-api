(function() {
    angular
        .module('app')
        .factory('authzInterceptor', authzInterceptor)
        .config(configuration);


    configuration.$inject = [
        '$urlRouterProvider',
        '$locationProvider',
        '$httpProvider',
        '$qProvider'
    ];

    function configuration($urlRouterProvider, $locationProvider, $httpProvider, $qProvider) {
        $urlRouterProvider.otherwise('/');
        $locationProvider.html5Mode(true);

        $httpProvider.interceptors.push('authzInterceptor');
        $qProvider.errorOnUnhandledRejections(false);
    };


    authzInterceptor.$inject = ['$cookies'];
    function authzInterceptor($cookies) {
        var access_token = $cookies.get('access_token');

        return {
            request: function(config) {
                /* append token if we have one and we're calling /api */
                if (config.url.startsWith('/api') && access_token) {
                    config.headers.Authorization = 'Bearer '+ access_token;
                }

                return config;
            },

            responseError: function(rejection) {
                /* only handle /api calls */
                if (!rejection.config.url.startsWith('/api')) {
                    return rejection;
                }

                console.log('!ERR! rejection', rejection);

                /* if access denied, redirect to login */
                if (rejection.status == 401) {
                    /* maybe attempt to get another token */
                    window.location = '/login';
                }

                return rejection;
            }
        };
    }
})();