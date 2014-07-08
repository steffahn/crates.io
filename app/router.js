import Ember from 'ember';

var Router = Ember.Router.extend({
  location: CargoENV.locationType
});

Router.map(function() {
  this.resource('login');
  this.resource('logout');
  this.resource('github_authorize', { path: '/authorize/github' });
  this.resource('packages');
  this.resource('package', { path: '/package/:package_id' }, function() {
    this.route('edit');
  });
});

export default Router;
