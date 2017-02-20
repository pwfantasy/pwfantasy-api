(function() {
    "use strict";
    const gulp = require('gulp');
    const path = require('path');
    const exec = require('child_process').exec;

    let plugins = require('gulp-load-plugins')({
        pattern: ['*'],
        scope: ['devDependencies'], // which keys in the config to look within
        replaceString: /^gulp(-|\.)/, // what to remove from the name of the module when adding it to the context
        rename: {
            'gulp-angular-order': 'angularOrder', //just an alias
            'gulp-angular-templatecache': 'angularTC' //just an alias
        }
    });

    let paths = {
        scripts: ['src/client/app/**/*.js'],
        styles: ['src/client/**/*.scss'],
        dist: 'dist',
    };

    gulp.task('ng', ['copy'], function() {
        return gulp.src('src/client/**/*.html')
            .pipe(plugins.angularTC({
                module: 'app',
                moduleSystem: 'IIFE',
                standAlone: false,
                filename: 'templates.config.js',
                transformUrl: function(url) {
                    return '/'+ url;
                }
            }))
            .pipe(gulp.dest(
                path.join(paths.dist, 'app')
            ));
    });

    gulp.task('inject', ['copy', 'ng', 'sass'], function () {
        let files = [
            'dist/app/**/*.js',
            'dist/app/**/*.css'
        ];

        let opts = {
            ignorePath: 'dist',
        };

        let order = {
            types: ['module', 'service', 'factory', 'controller', 'directive', 'filter', 'route', 'config'],
            special: ['/app']
        }

        return gulp.src('src/client/index.html')
            .pipe(plugins.inject(
                gulp.src(files[0])
                    .pipe(plugins.angularOrder(order)),
                opts
            ))
            .pipe(plugins.inject(
                gulp.src(files[1], {read: false}),
                opts
            ))
            .pipe(gulp.dest(paths.dist));
    });

    // copy all relevant files to dist
    gulp.task('copy', function() {
        //let del = plugins.del;
        plugins.del.sync([path.join(paths.dist, '/**/*')]);

        let files = [
            'src/client/**/*',
            '!src/client/**/*.scss',
            '!src/client/**/*.html',
        ];

        return gulp.src(files)
            .pipe(plugins.changed(paths.dist))
            .pipe(gulp.dest(paths.dist));
    });

    gulp.task('clean', function() {});
    gulp.task('uglify', function() {});
    gulp.task('reload', function() {});

    gulp.task('sass', function() {
        return gulp.src(paths.styles)
            .pipe(plugins.sass({
                outputStyle: 'compressed',
                sourceComments: true,
                sourceMap: true
            }))
            .pipe(gulp.dest(paths.dist));
    });

    gulp.task('watch', ['inject'], function() {
        let watch = gulp.watch('src/**/*', ['copy', 'ng', 'sass', 'inject']);
        watch.on('change', function(event) {
            console.log('File ' + event.path + ' was ' + event.type + ', running tasks...');
        });
    });

    gulp.task('server', ['inject'], function() {
        exec('node src/server/server', function (err, stdout, stderr) {
            console.log(stdout);
            console.log(stderr);
            cb(err);
        });
    });

    gulp.task('build', ['clean', 'uglify','reload']);
    gulp.task('default', ['copy', 'ng', 'sass', 'inject', 'watch', 'server']);
})();
