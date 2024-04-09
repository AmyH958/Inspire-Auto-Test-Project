import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import static com.kms.katalon.core.testobject.ObjectRepository.findWindowsObject
import com.kms.katalon.core.checkpoint.Checkpoint as Checkpoint
import com.kms.katalon.core.cucumber.keyword.CucumberBuiltinKeywords as CucumberKW
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile
import com.kms.katalon.core.model.FailureHandling as FailureHandling
import com.kms.katalon.core.testcase.TestCase as TestCase
import com.kms.katalon.core.testdata.TestData as TestData
import com.kms.katalon.core.testng.keyword.TestNGBuiltinKeywords as TestNGKW
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import com.kms.katalon.core.windows.keyword.WindowsBuiltinKeywords as Windows
import internal.GlobalVariable as GlobalVariable
import org.openqa.selenium.Keys as Keys

WebUI.openBrowser('')

WebUI.navigateToUrl('http://10.11.20.15/inspireapp/logon')

WebUI.maximizeWindow()

WebUI.setText(findTestObject('Object Repository/查詢/進階查詢/依瀏覽條件搜尋/Page_(15trunk)/input_(15trunk)_member_id'), 'catc')

WebUI.setEncryptedText(findTestObject('Object Repository/查詢/進階查詢/依瀏覽條件搜尋/Page_(15trunk)/input_(15trunk)_member_pwd'), 'eh9+mX6n5G+m9qEgDyu51A==')

WebUI.click(findTestObject('Object Repository/查詢/進階查詢/依瀏覽條件搜尋/Page_(15trunk)/input_(15trunk)_Submit'))

WebUI.verifyElementText(findTestObject('Object Repository/查詢/進階查詢/依瀏覽條件搜尋/Page_(15trunk)/a_'), '查詢')

WebUI.click(findTestObject('Object Repository/查詢/進階查詢/依瀏覽條件搜尋/Page_(15trunk)/a_'))

WebUI.verifyElementText(findTestObject('Object Repository/查詢/進階查詢/依瀏覽條件搜尋/Page_(15trunk)/a__1'), '進階查詢')

WebUI.click(findTestObject('Object Repository/查詢/進階查詢/依瀏覽條件搜尋/Page_(15trunk)/a__1'))

WebUI.verifyElementText(findTestObject('Object Repository/查詢/進階查詢/依瀏覽條件搜尋/Page_(15trunk)/b_'), '瀏覽')

WebUI.click(findTestObject('Object Repository/查詢/進階查詢/依瀏覽條件搜尋/Page_(15trunk)/img__ImageBrowse'))

WebUI.verifyElementText(findTestObject('Object Repository/查詢/進階查詢/依瀏覽條件搜尋/Page_(15trunk)/b__1'), '瀏覽條件:')

WebUI.verifyElementText(findTestObject('Object Repository/查詢/進階查詢/依瀏覽條件搜尋/Page_(15trunk)/select_ISBNISSN'), '正題名\n書名\n作者\n出版者/書商/捐贈者\n出版地\n出版年\nISBN/ISSN\n主題\n條碼號\n集叢名\n書目建立日期\n修改日期\n館藏登收日期\n索書號\n分類號\n書目建立館員姓名\n書目修改館員姓名\n系統識別號\n舊系統識別碼\n館員註記\n流通註記\n採購註記\n期刊註記\n官書編號\n叢刊代碼\n附註項\n版本項\n其他資料識別號\n其他號碼')

WebUI.setText(findTestObject('Object Repository/查詢/進階查詢/依瀏覽條件搜尋/Page_(15trunk)/input__listField'), '健康')

WebUI.verifyElementText(findTestObject('Object Repository/查詢/進階查詢/依瀏覽條件搜尋/Page_(15trunk)/input__browse'), '')

WebUI.click(findTestObject('Object Repository/查詢/進階查詢/依瀏覽條件搜尋/Page_(15trunk)/input__browse'))

WebUI.takeFullPageScreenshotAsCheckpoint('依條件搜尋正確顯示結果')

WebUI.click(findTestObject('Object Repository/查詢/進階查詢/依瀏覽條件搜尋/Page_(15trunk)/a__1_2'))

WebUI.closeBrowser()

