(function() {
    "use strict";

    angular
        .module('superstars.search')
        .config(configuration);

    function configuration($stateProvider) {
        $stateProvider
            .state('superstars.search', {
                url: '/search/:term',
                templateUrl: '/app/modules/superstars/search/search.view.html',
                controller: 'SearchCtrl',
                controllerAs: 'searchVm',
                resolve: {
                    term: getTerm,
                    results: getSearchResults
                }
            });

        getTerm.$inject = ['$stateParams'];
        function getTerm($stateParams) {
            return $stateParams.term;
        }

        getSearchResults.$inject = ['term', '$http'];
        function getSearchResults(term, $http) {
            return new Promise((resolve, reject) => {
                function _success(response) {
                    resolve(response.data || []);
                }

                function _error(response) {
                    reject(response);
                }

                var req = {
                    method: 'GET',
                    url: `/api/superstar/search/${term}`,
                    headers: {
                        'Content-Type': 'application/json'
                    },
                    data: ''
                }

                $http(req).then(_success, _error);
            });
        }
    }
})();