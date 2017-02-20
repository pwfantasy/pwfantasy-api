(function() {
    "use strict";

    angular
        .module('services.user')
        .service('UserService', UserService);

    UserService.$inject = ['$http', '$cookies'];

    function UserService($http, $cookies) {

        var service = this;

        service.isLoggedIn = function() {
            return true;
        };

        service.isAdmin = function() {
            return true;
        };

        service.getUser = function() {
            return new Promise(function(resolve, reject) {

                if (service._user) {
                    return resolve(service._user);
                }

                let url = '/api/user';
                let req = { method: 'GET', url };

                $http(req).then(_success, _fail);

                function _success(response) {
                    service._user = response.data;
                    resolve(response.data);
                }

                function _fail(response) {
                    // 401 issues will be automatically handled elsewhere
                    // we should account for 404, 500, etc. here
                }
            });
        };

        return service;
    }

})();

