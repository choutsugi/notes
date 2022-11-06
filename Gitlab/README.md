## CI/CD

CI（Continuous Integration），即持续整合。

CD（Continuous Delivery/Deployment），即持续交付/部署。

主流CI/CD平台：

- Jenkins：仅CI/CD。
- Gitlab：git服务器 与CI/CD

## 建立.gitlab-ci.yml

```yaml
helloworld:
  script:
    - echo "hello world, Gitlab!"
```

## 新增Pipeline Job

1. 新建项目：https://gitlab.com/projects/new#blank_project
2. 添加SSH公钥到Gitlab。
3. 依照项目下提示推送代码到Gitlab。
4. 自动触发CI/CD。