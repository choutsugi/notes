## URL构建器

### URL构建器

```go
package url_builder

import (
	"net/url"
)

// UrlBuilder build url with query
type UrlBuilder struct {
	baseURL *url.URL
	query   url.Values
}

func NewUrlBuilder(baseUrl string) (*UrlBuilder, error) {

	baseURL, err := url.Parse(baseUrl)
	if err != nil {
		return nil, err
	}

	query, err := url.ParseQuery(baseURL.RawQuery)
	if err != nil {
		return nil, err
	}

	return &UrlBuilder{
		baseURL,
		query,
	}, nil
}

func (builder *UrlBuilder) AddParam(key string, value string) *UrlBuilder {

	builder.query.Add(key, value)
	return builder
}

func (builder *UrlBuilder) Build() *url.URL {

	builder.baseURL.RawQuery = builder.query.Encode()
	return builder.baseURL
}

func (builder *UrlBuilder) BuildString() string {

	builder.baseURL.RawQuery = builder.query.Encode()
	return builder.baseURL.String()
}
```

### 测试

```go
func TestUrlBuilder(t *testing.T) {
	baseUrl := "https://unknowhost.com"
	builder, err := url_builder.NewUrlBuilder(baseUrl)
	if err != nil {
		t.Fatalf(err.Error())
	}
	reqUrl := builder.
		AddParam("state_type", "uuid").
		AddParam("channel", "unknown").
		AddParam("state", uuid.NewString()).
		BuildString()
	fmt.Println(reqUrl)
}
```
测试结果：
```text
=== RUN   TestUrl
https://unknowhost.com?channel=unknown&state=0b2c1f82-1ddb-4e51-b185-a3e57861251e&state_type=uuid
--- PASS: TestUrl (0.00s)
PASS
```
