# coding: utf-8
lib = File.expand_path('../lib', __FILE__)
$LOAD_PATH.unshift(lib) unless $LOAD_PATH.include?(lib)
require 'rmpd_adschedule/version'

Gem::Specification.new do |spec|
  spec.name          = 'rmpd_adschedule'
  spec.version       = RmpdAdschedule::VERSION
  spec.authors       = ['Oleg Antonyan']
  spec.email         = ['oleg.b.antonyan@gmail.com']

  spec.summary       = 'Write a short summary, because Rubygems requires one.'
  spec.description   = 'Write a longer description or delete this line.'
  # spec.homepage      = "Put your gem's website or public repo URL here."
  spec.license       = 'MIT'

  spec.files         = `git ls-files -z`.split("\x0").reject { |f| f.match(%r{^(test|spec|features)/}) }
  spec.bindir        = 'exe'
  spec.executables   = spec.files.grep(%r{^exe/}) { |f| File.basename(f) }
  spec.require_paths = ['lib']

  spec.extensions = Dir['rust/extconf.rb']

  spec.add_development_dependency 'bundler', '~> 1.11'
  spec.add_development_dependency 'rake', '~> 10.0'
  spec.add_development_dependency 'rspec', '~> 3.0'
end
