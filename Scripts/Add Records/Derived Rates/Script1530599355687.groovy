import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import com.kms.katalon.core.checkpoint.Checkpoint as Checkpoint
import com.kms.katalon.core.checkpoint.CheckpointFactory as CheckpointFactory
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as MobileBuiltInKeywords
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile
import com.kms.katalon.core.model.FailureHandling as FailureHandling
import com.kms.katalon.core.testcase.TestCase as TestCase
import com.kms.katalon.core.testcase.TestCaseFactory as TestCaseFactory
import com.kms.katalon.core.testdata.TestData as TestData
import com.kms.katalon.core.testdata.TestDataFactory as TestDataFactory
import com.kms.katalon.core.testobject.ObjectRepository as ObjectRepository
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WSBuiltInKeywords
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUiBuiltInKeywords
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import internal.GlobalVariable as GlobalVariable
import org.openqa.selenium.Keys as Keys

WebUI.openBrowser('')

WebUI.navigateToUrl('http://mauw2k8sdworx:81/')

WebUI.doubleClick(findTestObject('Screens/Derived Rates/input_Username'))

WebUI.setText(findTestObject('Screens/Derived Rates/input_Username'), 'dev@sdworx.com')

WebUI.doubleClick(findTestObject('Screens/Derived Rates/input_Password'))

WebUI.setText(findTestObject('Screens/Derived Rates/input_Password'), '1234')

WebUI.doubleClick(findTestObject('Screens/Derived Rates/input_Submit'))

WebUI.doubleClick(findTestObject('Screens/Derived Rates/h4_YOUR SETUP'))

WebUI.click(findTestObject('Screens/Derived Rates/button_Your Setup'))

WebUI.click(findTestObject('Screens/Derived Rates/a_Your Pay  Benefits'))

WebUI.click(findTestObject('Screens/Derived Rates/a_Derived Rates'))

WebUI.selectOptionByValue(findTestObject('Screens/Derived Rates/select_111111 - test981078 - D'), 'e865745f-ca2b-4af8-a580-50285c6b5440', 
    true)

WebUI.doubleClick(findTestObject('Screens/Derived Rates/a_Derived Rates_1'))

WebUI.click(findTestObject('Screens/Derived Rates/a_Add Derived Rate'))

WebUI.click(findTestObject('Screens/Derived Rates/div_Add Derived Rate'))

WebUI.selectOptionByValue(findTestObject('Screens/Derived Rates/select_Please select0100200300'), '100', true)

WebUI.doubleClick(findTestObject('Screens/Derived Rates/input_Description0'))

WebUI.setText(findTestObject('Screens/Derived Rates/input_Description0'), 'Test3')

WebUI.selectOptionByValue(findTestObject('Screens/Derived Rates/select_Please select0100200300_1'), '150', true)

WebUI.setText(findTestObject('Screens/Derived Rates/input_Value10'), '3.0000')

WebUI.click(findTestObject('Screens/Derived Rates/button_Save'))

//WebUI.closeBrowser()

